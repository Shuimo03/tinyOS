use core::panic;
use std::process::exit;

#[no_mangle]
#[link_section = ".text.entry"]

pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic("unreachable after sys_exit!");
}