#![allow(dead_code)]
#![cfg_attr(target_os = "none", no_std)]

extern crate core;

#[cfg_attr(target_os = "none", panic_handler)]
fn panic(_info: &::core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

pub fn main() {}
