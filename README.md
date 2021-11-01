# GoOS

GoOS使用Go语言编写的操作系统内核，使用qemu模拟RISC。

## 开发环境

+ qemu
+ WSL2 Ubuntu20.04
+ Go 1.17

## 启动

在boot下输入以下命令:

qemu-system-riscv32 -nographic -smp 4 -machine virt -bios none -kernel os.elf