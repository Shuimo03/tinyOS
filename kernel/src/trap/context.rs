use core::mem::zeroed;

use riscv::register::sstatus::{Sstatus};

#[repr(C)]
#[derive(Debug)]

pub struct TrapContext{
    pub register:[usize;32], //32个通用寄存器 x0-x31
    pub sstatus:Sstatus, // 保存状态位特权寄存器
    pub sepc: usize, //保存中断地址的特权状态寄存器
}

impl Default for TrapContext {
    fn default() -> Self {
        unsafe {
            zeroed()
        }
    }
}