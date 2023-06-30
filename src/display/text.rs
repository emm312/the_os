use core::fmt::{Write, self};

use lazy_static::lazy_static;
use noto_sans_mono_bitmap::{get_raster, FontWeight, RasterHeight, RasterizedChar};
use spin::Mutex;

use self::font_constants::BACKUP_CHAR;

use super::framebuffer::FRAMEBUFFER;

lazy_static! {
    pub static ref WRITER: Mutex<TextWriter> = Mutex::new(TextWriter::new());
}

mod font_constants {
    use noto_sans_mono_bitmap::get_raster_width;

    use super::*;
    pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;

    pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_RASTER_HEIGHT);

    pub const BACKUP_CHAR: char = 'E';
}

const VERT_SPACE: usize = 2;
const LETTER_SPACE: usize = 0;
const BORDER_PADDING: usize = 1;


pub struct TextWriter {
    col: usize,
    row: usize,
}

impl TextWriter {
    pub fn new() -> TextWriter {
        TextWriter { col: 0, row: 0 }
    }

    pub fn newline(&mut self) {
        self.row += font_constants::CHAR_RASTER_HEIGHT.val()+VERT_SPACE;
        self.cr();
    }

    fn get_raster(&self, c: char) -> RasterizedChar {
        get_raster(c, FontWeight::Regular, RasterHeight::Size16)
            .unwrap_or(get_raster(BACKUP_CHAR, FontWeight::Regular, RasterHeight::Size16).unwrap())
    }

    pub fn cr(&mut self) {
        self.col = 0;
    }

    pub fn clear(&mut self) {
        let lock = FRAMEBUFFER.lock();

        for i in 0..lock.ptr.width {
            for j in 0..lock.ptr.height {
                lock.set_pixel(i, j, 0);
            }
        }
    }

    fn write_rendered_char(&mut self, rendered_char: RasterizedChar) {
        let lock = FRAMEBUFFER.lock();
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                let n_b = *byte as u32;
                let new_byte = n_b|n_b<<8|n_b<<16;
                lock.set_pixel(x as u64 + self.col as u64, y as u64 + self.row as u64, new_byte);
            }
        }
        self.col += rendered_char.width() + LETTER_SPACE;
    }

    pub fn write_char(&mut self, c: char) {
        let lock = FRAMEBUFFER.lock();
        let width = lock.ptr.width.clone();
        let height = lock.ptr.height.clone();
        unsafe { FRAMEBUFFER.force_unlock(); }
        match c {
            '\n' => { self.newline(); self.cr(); },
            '\r' => { self.cr(); },
            _ => {
                let new_xpos = self.col + font_constants::CHAR_RASTER_WIDTH;
                if new_xpos >= width as usize {
                    self.newline();
                }
                let new_ypos =
                    self.row + font_constants::CHAR_RASTER_HEIGHT.val() + BORDER_PADDING;
                if new_ypos >= height as usize {
                    self.clear();
                }
                self.write_rendered_char(self.get_raster(c));
            }
        }
    }

}


impl Write for TextWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::display::text::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args);
}