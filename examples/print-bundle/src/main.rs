use esbuild_bundle::javascript;

fn main() {
    let path = javascript!("scripts/my-module.js" => "bundles");
    println!("output path: {path}");
}
