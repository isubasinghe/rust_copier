#![no_std]
#![no_main]
#![feature(panic_info_message, lang_items, fn_align)]
extern crate buddy_system_allocator;
extern crate alloc;
extern crate derive_more;
mod plat;
mod arch;
mod kmalloc;
mod mem;

#[no_mangle]
static mut OS_ELF_SZ: usize = 0xDEADBEEF;


#[no_mangle]
extern "C" fn kmain()  -> ! {
    kmalloc::setup_heap(); 
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
