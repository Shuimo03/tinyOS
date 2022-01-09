use super::{context::{TrapContext}, timer};
use core::{arch::global_asm};
use riscv::register::{
    scause::{Exception, Interrupt, Scause, Trap},
    sie, stvec,
};

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

        //开启外部中断使能
        sie::set_sext();
    }
}


//中断入口
#[no_mangle]
pub fn handle_trap(context: &mut TrapContext, scause: Scause, stval: usize){
    match scause.cause() {
        
        //断点中断
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        //时钟中断
        Trap::Interrupt(Interrupt::SupervisorTimer)=>supervisor_timer(context),
        //其他情况,终止当前线程
        _=>fault(context,scause,stval),
    };
}
/// 继续执行，其中 `sepc` 增加 2 字节，以跳过当前这条 `ebreak` 指令
fn breakpoint(context: &mut TrapContext){
    println!("Breakpoint at 0x{:x}",context.sepc);
    context.sepc += 2;
}

//处理时钟中断
fn supervisor_timer(_context: &mut TrapContext){
    timer::tick();
}

//出现不能解决的情况
fn fault(context: &mut TrapContext, scause: Scause, stval: usize) {
    panic!(
        "Unresolved interrupt: {:?}\n{:x?}\nstval: {:x}",
        scause.cause(),
        context,
        stval
    );
}