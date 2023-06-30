use limine::{LimineFramebuffer, LimineFramebufferRequest, NonNullPtr};
use spin::Mutex;

use crate::{hcf, serial_println};
use lazy_static::lazy_static;

static FRAMEBUFFER_REQUEST: LimineFramebufferRequest = LimineFramebufferRequest::new(0);

lazy_static! {
    pub static ref FRAMEBUFFER: Mutex<Framebuffer> = Mutex::new(Framebuffer::init());
}

pub struct Framebuffer {
    pub ptr: &'static NonNullPtr<LimineFramebuffer>,
}

unsafe impl Send for Framebuffer {}

impl Framebuffer {
    pub fn init() -> Framebuffer {
        if let Some(resp) = FRAMEBUFFER_REQUEST.get_response().get() {
            if resp.framebuffer_count < 1 {
                hcf();
            }

            let buffer = &resp.framebuffers()[0];

            Framebuffer { ptr: buffer }
        } else {
            hcf();
        }
    }

    pub fn set_pixel(&self, x: u64, y: u64, colour: u32) {
        unsafe {
            *(self
                .ptr
                .address
                .as_ptr()
                .unwrap()
                .offset((y * self.ptr.pitch) as isize + (x * 4) as isize)
                as *mut u32) = colour;
        }
    }
}
