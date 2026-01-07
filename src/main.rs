#![no_std]
#![no_main]

use lilith_kernal::printk::printk;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        printk(b"Hello World from Rust Kernel!\n\0".as_ptr() as *const i8);
    }
    loop {}
}
