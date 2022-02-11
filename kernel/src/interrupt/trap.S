.altmacro
# 宏：将寄存器存到栈上
.macro SAVE_GP, n
    sd x\n, \n*8(sp)
.endm
# 宏：将寄存器从栈中取出
.macro LOAD_GP, n
    ld  x\n, \n*8(sp)
.endm
    .section .text.trampoline
    .globl __trap     # 进入中断,保存 Context 并且进入 Rust 中的中断处理函数 trap::handler::handle_trap()
    .globl __restore  # 从 Context 中恢复所有寄存器，并跳转至 Context 中 sepc 的位置
    .align 2          # 将__trap的地址4字节对齐

 
 # 在栈上开辟 Context 所需的空间
__trap:
    csrw sp, sscratch,sp  #csrw写控制状态寄存器
    addi    sp, sp, -34*8 #34*8字节栈帧，保存Trap上下文。
    
    #保存通用寄存器x0-x31
    sd x1, 1*8(sp)
    sd x3, 3*8(sp)
    #跳过x4和x0====>TODO 补充原因
    #保存x5-x31
    .set n,5
    .rept 27
        SAVE_GP %n
        .set n,n+1
    .endr

    # 取出 CSR 并保存
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # read user stack from sscratch and save it on the kernel stack
    csrr t2, sscratch
    sd t2, 2*8(sp)
    # set input argument of trap_handler(cx: &mut TrapContext)
    mv a0, sp
    call trap_handler

# 离开中断,从栈上恢复的Trap上下文恢复
__restore:

    # 从内核栈顶的 Trap 上下文恢复通用寄存器和 CSR,先恢复CSR再恢复通用寄存器。
    mv sp,  a0

    ld t0,  32*8(sp)
    ld t1,  33*8(sp)
    ld t2, 2*8(sp)

    csrw sstatus,t0
    csrw sepc,t1
    csrw sscratch,t2

    ld x1,  1*8(sp)
    ld x3,  3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr

    addi sp,sp 34*8
    csrrw sp,sscratch,sp
    sret    #回到U特权级继续应用程序控制流