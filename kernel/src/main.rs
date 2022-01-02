#![no_std]
#![no_main] //内嵌整个汇编文件
#![feature(panic_info_message)]


#[macro_use]
mod console;
mod panic;
mod sbi;
use core::arch::global_asm;

// 内嵌汇编
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> !{
    println!("====TinyOs===");
    panic!("end of rust_main");
}