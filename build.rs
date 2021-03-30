extern crate embed_resource;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        embed_resource::compile("build/windows/icon.rc");
    }
}
