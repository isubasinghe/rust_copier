#![no_std]
#![no_main]
#![feature(panic_info_message, lang_items, fn_align, core_intrinsics)]


#[lang = "eh_personality"]
extern "C" fn eh_personality() {}


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

pub fn init() {
}

#[no_mangle]
pub extern "C" fn init() {

}

#[no_mangle]
pub extern "C" fn protected() {

}

#[no_mangle]
pub extern "C" fn notified(idx: core::ffi::c_uint) {

}

#[cfg(test)]
mod test {
    fn test_runner() {
    }
}
