# lab os1
##实现步骤
1. 将程序的目标平台换成 riscv64gc-unknown-none-elf,即裸机平台
2. 标准库 std 的引用换成核心库 core。
3. 移除 println! 宏
4. 子模块 lang_items.rs，在里面编写 panic 处理函数，通过标记 #[panic_handler] 告知编译器采用我们的实现
5. 移除 main 函数
6. 给 Rust 编译器提供入口函数 _start()
7. 调整内核的内存布局 链接脚本
8. 剥离多余的元数据得到内核镜像并提供给 Qemu
9. 基于 GDB 验证启动流程
10. 置好栈使得内核代码可以正常进行函数调用，随后将控制权转交给 Rust 代码。
11. 用SBI提供的服务完成print和关机
    
>SBI是操作系统的引导程序和运行时。机器上电时，SBI将配置环境，准备设备树，最终将引导启动操作系统。
操作系统需要访问硬件或者特殊的功能，这时候就需要通过ecall指令陷入M层的SBI运行时，由SBI完成这些功能再提供。
SBI的实现体现了RISC-V层级和模块化的开发特点，我们可以方便地更换SBI实现，无需重装操作系统，就能支持M层不同的功能，
比如无盘机、网络启动和管理等等，都是更高级的SBI实现能完成的。

## 代码树
>├── bootloader (内核依赖的运行在 M 特权级的 SBI 实现，本项目中我们使用 RustSBI)
│   └── rustsbi-qemu.bin
├── os
│   ├── Cargo.toml (cargo 项目配置文件)
│   ├── Makefile
│   └── src
│       ├── console.rs (将打印字符的 SBI 接口进一步封装实现更加强大的格式化输出)
│       ├── entry.asm (设置内核执行环境的的一段汇编代码)
│       ├── lang_items.rs (需要我们提供给 Rust 编译器的一些语义项，目前包含内核 panic 时的处理逻辑)
│       ├── linker.ld (控制内核内存布局的链接脚本以使内核运行在 qemu 虚拟机上)
│       ├── logging.rs (为本项目实现了日志功能)
│       ├── main.rs (内核主函数)
│       └── sbi.rs (封装底层 SBI 实现提供的 SBI 接口)
└── rust-toolchain.toml (整个项目的工具链版本)