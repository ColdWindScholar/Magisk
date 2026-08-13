use crate::cxx_extern::*;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn xpipe2(fds: &mut [i32; 2], flags: i32) -> i32;
    }
}
