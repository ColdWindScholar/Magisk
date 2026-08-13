use std::env;

use crate::codegen::{gen_cxx_binding, gen_cxx_binding_from};

#[path = "../include/codegen.rs"]
mod codegen;

fn main() {
    gen_cxx_binding("base-rs");
    let target = env::var("TARGET").unwrap();
    if target.contains("android")
        || target.contains("linux")
        || target.contains("cygwin")
    {
        gen_cxx_binding_from("linux_like.rs", "base-linuxlike-rs");
    }
}
