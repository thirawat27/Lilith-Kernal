use crate::types::*;

use crate::printk::printk;

#[no_mangle]
pub unsafe extern "C" fn panic(s: *const c_char) -> ! {
    printk(b"Kernel panic: %s\n\r\0".as_ptr() as *const c_char, s);
    loop {}
}
