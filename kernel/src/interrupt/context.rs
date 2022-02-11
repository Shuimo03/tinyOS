use riscv::register::sstatus::{Sstatus, self, SPP};

#[repr(C)]
pub struct TrapContext{
    pub register:[usize;32], //32个通用寄存器 x0-x31
    pub sstatus:Sstatus, // 保存状态位特权寄存器
    pub sepc: usize, //保存中断地址的特权状态寄存器
}


impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) { self.register[2] = sp; }
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            register: [0; 32],
            sstatus,
            sepc: entry,
        };
        cx.set_sp(sp);
        cx
    }
}