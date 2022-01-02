// 代替std库,实现panic和abort的功能

use core::panic::PanicInfo;
use crate::sbi::shutdown;

#[panic_handler]
fn panic_handler(info: &PanicInfo)->! {
    println!("\x1b[1;31mpanic: '{}'\x1b[0m", info.message().unwrap());
    shutdown()
}

#[no_mangle]
extern "C" fn abort()->!{
    panic!("abort()")
}