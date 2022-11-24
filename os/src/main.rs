#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod console;
mod lang_items;
mod sbi;

//云原生

use core::arch::global_asm;
//通过 include_str! 宏将同目录下的汇编代码 entry.asm 转化为字符串并通过 global_asm! 宏嵌入到代码中。
global_asm!(include_str!("entry.asm"));
//先是几句汇编
//然后跳到80000000 SBI 类似于什么
//-Start
//然后交给80200000 entry的Qemu 交给内核

//内核需要通过另一种复杂的方式来“调用” RustSBI 的服务：

//把控制权交给rust
#[no_mangle]
pub fn rust_main()->!{
    clear_bss();
    println!("Hello, world!");
    panic!("Shutdown machine!");
}
fn clear_bss(){
    extern "C"{
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a|{
        unsafe{(a as *mut u8).write_volatile(0)}
    });
}