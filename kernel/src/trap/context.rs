use riscv::register::{sstatus::Sstatus, scause::Scause};

#[repr(C)]
#[derive(Debug)]

pub struct Context{
    pub x:[uszie;32], // 32个通用寄存器
    pub sstatus: Sstatus,
    pub sepc: usize
}