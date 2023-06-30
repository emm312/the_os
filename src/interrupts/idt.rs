use lazy_static::lazy_static;
use x86_64::structures::{idt::{InterruptDescriptorTable, InterruptStackFrame}};
use crate::{interrupts::tss, serial_println};

use crate::println;

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(tss::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_idt() {
    IDT.load();
    x86_64::instructions::interrupts::int3();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}