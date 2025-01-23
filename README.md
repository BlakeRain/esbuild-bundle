# esbuild-bundle

This crate provides a very simple way of bundling JavaScript modules using [esbuild] with a
`javascript!` macro.

The `javascript!` expands to a literal string, and as such can be invoked wherever an expression
macro can be used. The macro takes an argument that is interpreted as a path to a JavaScript module.
The paths can include variable expansion, which can be useful when combined with the
`CARGO_MANIFEST_DIR` environment variable. The macro causes the JavaScript module to be bundled
using `esbuild` and writes the output to a _bundle directory_. The output bundle path is then output
from the macro.

The bundles directory is either specified directly in the `javascript!` macro invocation after a
`=>` symbol, or it is specified in a `esbuild-bundle.json` configuration file. As with the script
path, the bundle directory may also use environment variables such as `CARGO_MANIFEST_DIR`.

Here's a simple example. Let's assume that we have a `scripts/my-module.js` file relative to the
`Cargo.toml` file for our crate. We want to bundle this module into a `bundles` directory and then
print out the bundled path.

```rust
use esbuild_bundle::javascript;

pub fn main() {
    let path = javascript!("${CARGO_MANIFEST_DIR}/scripts/my-module.js" => "bundles");
    println!("output path: {path}");
}
```

When we compile this code, the JavaScript module in the `scripts` directory will get bundled into
the given bundles directory, and macro will expand to the output bundle. In this case, the bundles
directory was given as `bundles`.

During compilation, two files to be created in the `bundles` directory by the macro: the bundled
JavaScript file and a `.bundles.json` manifest file. The macro expands to the path to the bundled
JavaScript file as a string literal.

For example, the `scripts/my-module.js` module could get bundled to the file `bundles/eUmAqV.js`
(actual file name will be 32 characters). The script name (`eUmAqV.js`) is what will be printed out
by the program.

The `.bundles.json` manifest file is used to reduce replication, so that every input file is mapped
to the same random filename, rather than ending up with lots of bundles. Examining the
`.bundles.json` file we can see how this works for our example:

```json
{
  "/path/to/this/crate/scripts/my-module.js": "eUmAqV.js"
}
```

# Configuration

Rather than specify the bundle path in every call of the `javascript!` macro, you can specify a
`bundle_path` in the `esbuild-bundle.json` configuration file. This file is expected to be found
adjacent to the `Cargo.toml` file for your crate; that is to say, the macro looks for the
configuration file under `$CARGO_MANIFEST_DIR/esbuild-bundle.json`.

In the above example, where bundles are written to the `bundles` directory, this can be configured
as follows:

```json
{
    "bundle_path": "${CARGO_MANIFEST_DIR}/bundles"
}
```

If the bundle path does not exist, it will be created.

## Esbuild Command

Without any configuration, the macro assumes it can invoke `esbuild` by using [npx]: `npx esbuild`.
This can be changed to one of three other modes by specifying the `esbuild` configuration in the
`esbuild-bundle.json` configuration file.

```json
{
    "esbuild": {
        "type": "npx"
    }
}
```

The `esbuild` key expects to be an object with a `type` property, which supports the following
values:

- `"npx"` will use the `npx` command to invoke `esbuild`. This is the default behaviour.
- `"yarn"` will use the `yarn` command to invoke `esbuild`.
- `"npm"` will invoke a script from the `scripts` section of a `package.json` file. The script name
  is specified using the `script` property in the `esbuild` object. The script is passed two
  arguments: the path to the given entry-point (the JavaScript module) and the expected path to the
  output bundle file.
- `"pnpm"` will use `pnpm` to either invoke `esbuild` via `pnpm exec esbuild` or it can be used to
  run a script from the `scripts` section of a `package.json` file, similar to the `"npm"` option.

[esbuild]: https://esbuild.github.io/
[npx]: https://docs.npmjs.com/cli/v7/commands/npx
