#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

use core::panic::PanicInfo;
use the_os::{hcf, println, interrupts::{idt::init_idt, tss::load_gdt}, serial_println};

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);
    hcf();
}

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    load_gdt();
    init_idt();
    main();
    hcf();
}

fn main() {
    
}