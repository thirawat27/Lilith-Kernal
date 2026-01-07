#![no_std]
#![no_main]

use lilith_kernal::printk::printk;
use bootloader::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    unsafe {
        printk(b"Hello World from Rust Kernel!\n\0".as_ptr() as *const i8);
    }
    loop {}
}
