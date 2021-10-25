## 开发环境配置

### Go安装

因为我的开发环境是Ubuntu20.04(WSL2)，所以在官网中选择 go1.17.2linux-amd64.tar.gz

```
https://golang.google.cn/dl/
```

### qemu安装

安装依赖包

```
$ sudo apt-get update && sudo apt-get upgrade
$ sudo apt install autoconf automake autotools-dev curl libmpc-dev libmpfr-dev libgmp-dev \
              gawk build-essential bison flex texinfo gperf libtool patchutils bc \
              zlib1g-dev libexpat-dev pkg-config  libglib2.0-dev libpixman-1-dev git tmux python3
```

安装RISC-V

```
$ sudo apt-get install git build-essential gdb-multiarch qemu-system-misc gcc-riscv64-linux-gnu binutils-riscv64-linux-gnu
```

