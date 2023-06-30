#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod display;
pub mod interrupts;

use core::arch::asm;

use interrupts::idt::init_idt;

pub fn hcf() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}