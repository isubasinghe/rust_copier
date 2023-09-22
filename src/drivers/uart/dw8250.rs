use core::intrinsics::{volatile_store, volatile_load};
use crate::arch::cpu;


const LSR: usize = 0x14;
const THR: usize = 0x00;

const THRE: usize = 5;

pub struct DW8250 {
    addr: usize
}


impl DW8250 {

    fn init(&mut self) {
    }

    fn write(&mut self, data: *mut char, count: usize) {}
    unsafe fn putc(&mut self, data: char) {
        volatile_store((self.addr + THR) as *mut char, data);
        while (volatile_load((self.addr + LSR) as *mut u32) >> THRE) & 1 != 1 {
            cpu::relax();
        }
    }
}
