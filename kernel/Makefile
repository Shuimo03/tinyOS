# Building
TARGET  := riscv64gc-unknown-none-elf
MODE    := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/kernel
KERNEL_BIN := target/$(TARGET)/$(MODE)/kernel.bin
DISASM_TMP := target/$(TARGET)/$(MODE)/asm

OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

.PHONY: doc kernel build clean qemu run

# 默认 build 为输出二进制文件
build: env $(KERNEL_BIN) 

env:
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils --vers =0.3.3
	rustup component add rust-src
	rustup component add llvm-tools-preview

# 生成 kernel 的二进制文件
$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@


kernel:
	@cd ../user && make build
	@echo Platform: qemu
	@cp src/linker-qemu.ld src/linker.ld
	@cargo build --release
	@rm src/linker.ld

# 查看反汇编结果
asm:
	@$(OBJDUMP) -d $(KERNEL_ELF) | less

# 清理编译出的文件
clean:
	@cargo clean

# 运行 QEMU

# 一键运行
run: build qemu

qemu: build
	@qemu-system-riscv64 \
            -machine virt \
            -nographic \
            -bios default \
            -device loader,file=$(KERNEL_BIN),addr=0x80200000