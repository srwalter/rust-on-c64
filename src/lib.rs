#![no_std]

pub struct VIC;

impl VIC {
    pub fn set_border_color(color: u8) {
        unsafe {
            let border_color = 0xd020 as *mut u8;
            *border_color = color;
        }
    }
    
    pub fn get_screen_mem() -> &'static [u8] {
        unsafe {
            let screen_mem = 0x400 as *mut u8;
            core::slice::from_raw_parts(screen_mem, 2048)
        }
    }
}

pub fn main() {
    VIC::set_border_color(1);
    let mem = VIC::get_screen_mem();
    mem[0] = 'A' as u8;
}
