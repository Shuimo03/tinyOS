#[allow(unused)]
pub const PAGE_SIZE: usize = 4096; // 页大小,每页为4KB。
pub const PAGE_SIZE_BITS: usize = 12; // 页内偏移量
pub const KERNEL_END: usize = 0x80800000; // 总内存大小8MB[0x80000000-0x80800000),但是qemu的内存可以调整,这里主要是为了配合K210