#![no_std]
#![no_main]

use core::arch::asm;

pub fn hcf() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}