use super::context::TrapContext;
use riscv::register::stvec;
use core::arch::global_asm;
use riscv::register::scause::Scause;

global_asm!(include_str!("trap.S"));

// 初始化中断处理,把中断入口写入到stevec中，并开启中断使能。
pub fn init(){
    unsafe{
        extern "C"{
            /// `trap.S` 中的中断入口
            fn __trap();
        }
        //使用 Direct 模式，将中断入口设置为 `__trap`
        stvec::write(__trap as usize, stvec::TrapMode::Direct);
    }
}


//中断入口
#[no_mangle]
pub fn handle_trap(context: &mut TrapContext, scause: Scause, stval: usize){
    panic!("Interrupted: {:?}", scause.cause());
}