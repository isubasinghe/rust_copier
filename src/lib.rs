#![no_std]
#![no_main]
#![feature(panic_info_message, lang_items, fn_align, core_intrinsics)]
extern crate buddy_system_allocator;
extern crate alloc;
extern crate derive_more;
extern crate libos;
extern crate tock_registers;
mod plat;
mod arch;
mod kmalloc;
mod mem;
mod drivers;
use libos::uart::*;

#[no_mangle]
static mut OS_ELF_SZ: usize = 0xDEADBEEF;


#[no_mangle]
extern "C" fn kmain()  -> ! {
    let mut udev = NS16550::new(0x10000000);
    udev.put('H');
    udev.put('e');
    udev.put('l');
    udev.put('l');
    udev.put('o');
    udev.put('\r');
    udev.put('\n');
    // kmalloc::setup_heap(); 
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[no_mangle]
fn interrupt_handler() {}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // print!("Aborting: ");
    if let Some(p) = info.location() {
        /* println!(
            "line {}, file {}: {}",
            p.line(),
            p.file(),
            info.message().unwrap()
        ); */
    } else {
        // println!("no information available.");
    }
    abort();
}

fn abort() -> ! {
    loop {}
}

#[cfg(test)]
mod test {
    fn test_runner() {
    }
}
