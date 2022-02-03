#![no_std]
#![no_main] //内嵌整个汇编文件
#![feature(panic_info_message)]


#[macro_use]
mod console;
mod panic;
mod sbi;
mod sys;
mod batch;
mod sync;
mod config;
mod mm;
use core::arch::global_asm;
mod trap;

// 内嵌汇编
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn class_bss(){
    extern "C"{
        fn sbss();
        fn ebss();
    }
    unsafe{
        core::slice::from_raw_parts_mut(
            sbss as usize as *mut u8,
            ebss as usize - sbss as usize,
        ).fill(0);
    }
}

#[no_mangle]
pub extern "C" fn rust_main(){
    class_bss();
    println!("=============TinyOS===================");
    trap::init();
    batch::init();
    batch::run_next_app();
}