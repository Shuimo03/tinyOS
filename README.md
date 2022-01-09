# TinyOS

TinyOS是一个类Unix内核，TinyOS参考了XV6和rCore,最初版本是打算使用Go来写，后来转向Rust。代码分层如下:

+ doc: 这里存放开发过程中的文档。
+ kernel:内核态代码
+ user:用户态代码
+ rust-toolchain:限定 Rust 工具链版本

## 开发环境

+ qemu
+ WSL2 Ubuntu20.04


## 参考链接

+ https://github.com/Ko-oK-OS/xv6-rust
+ https://rcore-os.github.io/rCore-Tutorial-deploy/
+ https://rcore-os.github.io/rCore-Tutorial-Book-v3/
+ https://reberhardt.com/cs110l/spring-2021/