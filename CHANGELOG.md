# Changelog

All notable changes to this library will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/1.0.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.3](https://github.com/BlakeRain/esbuild-bundle/compare/v0.3.2..v0.3.3) - 2025-01-23

Another small change that adds a `working_dir` option to the configuration. This allows specifying
the working directory in which the `esbuild` command is run. This can use macro expansion, which
makes it easier to work with Cargo workspaces. For example, to set the working directory to the
directory containing the `Cargo.toml` file, use the following configuration:

```json
{
    "working_dir": "$CARGO_MANIFEST_DIR"
}
```

## [0.3.2](https://github.com/BlakeRain/esbuild-bundle/compare/v0.3.1..v0.3.2) - 2025-01-23

This is another small revision that adds support for running `pnpm exec` instead of `npx`. In
addition, `pnpm` can be run with a script, similar to `npm`.

Running `esbuild` via `pnpm exec` requires setting the `esbuild.type` to `pnpm`:

```json
{
    "esbuild": {
        "type": "pnpm",
    }
}
```

To run a script from the `scripts` section of a `package.json` using `pnpm` rather than `npm`, set
the `esbuild.type` to `pnpm` and supply a `script` field:

```json
{
    "esbuild": {
        "type": "pnpm",
        "script": "my_script"
    }
}
```


## [0.3.1](https://github.com/BlakeRain/esbuild-bundle/compare/v0.3.0..v0.3.1) - 2024-10-22

This is a small revision that stops the `javascript!` macro from creating the bundle directory
itself. This avoids issues where the bundle directory ends up creating empty directories when these
are not wanted.

## [0.3.0](https://github.com/BlakeRain/esbuild-bundle/compare/v0.2.0..v0.3.0) - 2024-10-

This major revision changes how the bundles are created, and allows the control of the bundle
format and the global name directly from the macro invocation.

```rust
let my_file = javascript!("path/to/file.js", format="esm", global_name="myGlobalName");
```

## [0.2.0](https://github.com/BlakeRain/esbuild-bundle/compare/v0.1.1..v0.2.0) - 2024-10-21

This is a major revision that generates both `.js` and `.min.js` with source maps (in `.map`) files.

## [0.1.2](https://git.blakerain.com/BlakeRain/esbuild-bundle/compare/v0.1.1..v0.1.2) - 2024-08-26

This is another small revision to address a bug caused since the addition of the environment
expansion.

### Fixed

- The bundle mapping (`.bundles.json` file) now associates the target path with the destination file
  name. Moreover this is what is returned from the `javascript!` macro, rather than the complete
  bundle path. Otherwise the returned paths are absolute when used with environment expansion.

## [0.1.1](https://git.blakerain.com/BlakeRain/esbuild-bundle/compare/v0.1.0..v0.1.1) - 2024-08-26

This is a small revision that adds a little functionality.

### Added

- Uses [shellexpand](https://docs.rs/shellexpand/latest/shellexpand/) to expand environment
  variables in both the bundle path and the path to source files.

