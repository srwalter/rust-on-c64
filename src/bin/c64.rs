#![feature(lang_items, core_intrinsics)]
#![feature(start)]
#![no_std]

extern crate c64_rust;

use core::intrinsics;

#[start]
fn start(_argc: i8, _argv: *const *const u8) -> i8 {
    c64_rust::main();
    0
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    unsafe { intrinsics::abort() }
}
