use esbuild_bundle::javascript;

fn main() {
    let path = javascript!("${CARGO_MANIFEST_DIR}/scripts/my-module.js" => "${CARGO_MANIFEST_DIR}/bundles");
    println!("output path: {path}");
}
