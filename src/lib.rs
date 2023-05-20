#![no_std]
#![no_main]
#![feature(panic_info_message, lang_items, fn_align)]

mod plat;

#[no_mangle]
extern "C" fn kmain() {
    plat::is_visionfive2();
}


#[lang = "eh_personality"]
extern "C" fn eh_personality() {}


#[no_mangle]
fn interrupt_handler() {
}

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

