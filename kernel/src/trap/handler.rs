use super::context::Context;
use riscv::register::stvec;

global_asm!(include_str!("./trap.S"));

// 初始化中断处理

pub fn init(){
    unsafe{
        extern "C"{
            /// `interrupt.asm` 中的中断入口
            fn __interrupt();
        }
        //使用 Direct 模式，将中断入口设置为 `__interrupt`
        strvec::write(__interrupt as usize, stvec::TrapMode::Direct);
    }
}

pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize){
    panic!("Interrupted: {:?}", scause.cause());
}