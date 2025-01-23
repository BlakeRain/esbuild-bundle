use std::{collections::HashMap, path::PathBuf, process::Command};

use proc_macro::TokenStream;
use quote::quote;
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;
use syn::{parse::Parse, Token};

#[derive(Default)]
struct EsbuildOptions {
    format: Option<String>,
    global_name: Option<String>,
}

impl EsbuildOptions {
    fn add(&self, command: &mut Command) {
        if let Some(format) = &self.format {
            command.arg(format!("--format={format}"));
        }

        if let Some(global_name) = &self.global_name {
            command.arg(format!("--global-name={global_name}"));
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum EsbuildCommand {
    Npm { script: String },
    Pnpm { script: Option<String> },
    Npx,
    Yarn,
}

impl Default for EsbuildCommand {
    fn default() -> Self {
        Self::Npx
    }
}

impl EsbuildCommand {
    fn build_command(
        &self,
        options: &EsbuildOptions,
        minified: bool,
        entry_point: &str,
        output_path: &str,
    ) -> Command {
        match self {
            Self::Npm { script } => {
                let mut command = Command::new("npm");
                command.arg(script).arg(entry_point).arg(output_path);
                command
            }

            Self::Pnpm { script } => {
                if let Some(script) = script {
                    let mut command = Command::new("pnpm");
                    command.arg(script).arg(entry_point).arg(output_path);
                    command
                } else {
                    let mut command = Command::new("pnpm");
                    command.arg("exec");
                    command.arg("esbuild");
                    options.add(&mut command);

                    if minified {
                        command.arg("--minify");
                    } else {
                        command.arg("--sourcemap");
                    }

                    command
                        .arg("--bundle")
                        .arg(format!("--outfile={output_path}"))
                        .arg(entry_point);

                    command
                }
            }

            Self::Npx => {
                let mut command = Command::new("npx");
                command.arg("esbuild");
                options.add(&mut command);

                if minified {
                    command.arg("--minify");
                } else {
                    command.arg("--sourcemap");
                }

                command
                    .arg("--bundle")
                    .arg(format!("--outfile={output_path}"))
                    .arg(entry_point);

                command
            }

            Self::Yarn => {
                let mut command = Command::new("yarn");
                command.arg("esbuild");
                options.add(&mut command);

                if minified {
                    command.arg("--minify");
                } else {
                    command.arg("--sourcemap");
                }

                command
                    .arg("--bundle")
                    .arg(format!("--outfile={output_path}"))
                    .arg(entry_point);

                command
            }
        }
    }
}

#[derive(Default, Deserialize)]
struct Config {
    bundle_path: Option<String>,
    esbuild: Option<EsbuildCommand>,
}

impl Config {
    fn load() -> Self {
        let manifest_dir =
            std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR to be defined");
        let config_path = format!("{manifest_dir}/esbuild-bundle.json");
        let config_path = PathBuf::from(config_path);

        if config_path.exists() && config_path.is_file() {
            let content = std::fs::read_to_string(&config_path).unwrap_or_else(|err| {
                panic!("Failed to read configuration '{config_path:?}': {err:?}");
            });

            match serde_json::from_str(&content) {
                Ok(config) => config,
                Err(err) => {
                    panic!("Failed to parse configuration '{config_path:?}': {err:?}");
                }
            }
        } else {
            Default::default()
        }
    }
}

struct JavascriptModule {
    entry_point: String,
    bundle_path: Option<String>,
    options: EsbuildOptions,
}

impl Parse for JavascriptModule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let entry_point = input.parse::<syn::LitStr>()?.value();

        let mut bundle_path = None;
        if input.parse::<Option<Token![=>]>>()?.is_some() {
            bundle_path = Some(input.parse::<syn::LitStr>()?.value());
        }

        let mut options = EsbuildOptions::default();

        // Next we have a comma-separated list of parameters in the form 'foo=bar', where 'foo' is
        // an identifier and 'bar' is a literal value. We then assign the value corresponding the
        // key.

        while !input.is_empty() {
            input.parse::<Token![,]>()?;

            let key: syn::Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "format" => {
                    options.format = Some(input.parse::<syn::LitStr>()?.value());
                }

                "global_name" => {
                    options.global_name = Some(input.parse::<syn::LitStr>()?.value());
                }

                _ => {
                    return Err(syn::Error::new(
                        key.span(),
                        format!("unexpected key '{}'", key),
                    ));
                }
            }
        }

        Ok(Self {
            entry_point,
            bundle_path,
            options,
        })
    }
}

pub(crate) fn process(input: TokenStream) -> TokenStream {
    // Parse the input tokens into our configuration structure.
    let JavascriptModule {
        entry_point,
        bundle_path,
        options,
    } = syn::parse_macro_input!(input as JavascriptModule);

    // Expand any environment variables in the entry point.
    let entry_point: String = shellexpand::env(&entry_point)
        .unwrap_or_else(|err| {
            panic!("Failed to expand environment variables in entry point: {err}");
        })
        .into();

    // Load our configuration (if we have one)
    let config = Config::load();

    // Ascertain the path to where bundles are written.
    let bundle_path = bundle_path.or(config.bundle_path).unwrap_or_else(|| {
        panic!("No bundle path provided, and either no configuration found or configuration has no 'bundle_path'");
    });

    // Expand any environment variables in the bundle path.
    let bundle_path: String = shellexpand::env(&bundle_path)
        .unwrap_or_else(|err| {
            panic!("Failed to expand environment variables in bundle path: {err}");
        })
        .into();

    // The path to the bundles cache is under the bundles path, with the name '.bundles.json'.
    let bundles_path = format!("{bundle_path}/.bundles.json");
    let bundles_path = PathBuf::from(bundles_path);

    // See if we have a bundles cache, and if we do, load it.
    let mut bundles = if bundles_path.is_file() {
        let content = std::fs::read_to_string(&bundles_path).unwrap_or_else(|err| {
            panic!("Failed to read bundles cache '{bundles_path:?}': {err:?}");
        });

        match serde_json::from_str::<HashMap<String, String>>(&content) {
            Ok(config) => config,
            Err(err) => {
                panic!("Failed to parse bundles cache '{bundles_path:?}': {err:?}");
            }
        }
    } else {
        HashMap::new()
    };

    // Figure out where we're going to write the output. If we already have a bundle for the given
    // entry point then we can just reuse that. Otherwise we want to generate a new bundle
    // identifier (a random sequence of letters and numbers).
    let output_name = if let Some(existing) = bundles.get(&entry_point) {
        existing.clone()
    } else {
        let output_name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        bundles.insert(entry_point.clone(), output_name.clone());
        output_name
    };

    // Generate the JavaScript bundles using esbuild.
    let cmd = config.esbuild.unwrap_or_default();
    for (minified, output_name) in [
        (false, format!("{}.js", output_name)),
        (true, format!("{}.min.js", output_name)),
    ] {
        let output_path = format!("{bundle_path}/{output_name}");
        let output = match cmd
            .build_command(&options, minified, &entry_point, &output_path)
            .output()
        {
            Ok(output) => output,
            Err(err) => {
                panic!("Failed to run esbuild: {err:?}");
            }
        };

        if !output.status.success() {
            panic!(
                "failed to run esbuild: {}\n{}",
                std::str::from_utf8(&output.stderr).unwrap_or("unable to parse stderr as utf-8"),
                std::str::from_utf8(&output.stdout).unwrap_or("unable to parse stdout as utf-8")
            );
        }
    }

    // Write the changes to the bundles.json file.
    if let Err(err) = std::fs::write(
        &bundles_path,
        serde_json::to_string_pretty(&bundles).expect("failed to serialize '.bundles.json"),
    ) {
        panic!("Failed to write bundles cache '{bundles_path:?}': {err:?}");
    }

    quote! { #output_name }.into()
}
