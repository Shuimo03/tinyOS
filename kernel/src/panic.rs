// 代替std库,实现panic和abort的功能
use core::panic::PanicInfo;
use crate::sbi::shutdown;

#[panic_handler]
fn panic_handler(info: &PanicInfo)->! {
   
    if let Some(location) = info.location(){
      println!("[kernel] Panicked at {}:{} {}",location.file(),location.line(),info.message().unwrap());
    }else{
        println!("[kernel] Panicked: {}", info.message().unwrap());
    }
    shutdown()
}