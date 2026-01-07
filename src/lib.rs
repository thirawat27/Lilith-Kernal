#![no_std]
#![feature(c_variadic)]

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub mod ctype;
pub mod types;
pub mod string;
pub mod errno;
pub mod unistd;
pub mod panic;
pub mod vsprintf;
pub mod printk;





