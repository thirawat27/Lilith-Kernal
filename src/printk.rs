use crate::types::*;
use crate::vsprintf::{vsprintf, VaList};
use core::arch::global_asm;

static mut PRINTK_BUF: [c_char; 1024] = [0; 1024];

#[no_mangle]
pub unsafe extern "C" fn rust_printk_impl(fmt: *const c_char, args_ptr: *const c_void) -> c_int {
    let args = VaList::from_ptr(args_ptr); 
    // Suppress static_mut_refs warning with a pointer cast if needed, or allow it.
    let buf_mut_ptr = &raw mut PRINTK_BUF as *mut [c_char; 1024] as *mut c_char;
    let len = vsprintf(buf_mut_ptr, fmt, args);
    
    // Write directly to VGA buffer
    let buf = &raw const PRINTK_BUF as *const [c_char; 1024] as *const u8;
    for i in 0..len {
        console_write_char(*buf.add(i as usize));
    }
    
    len
}

// Simple VGA buffer handling
static mut VGA_INDEX: usize = 0;
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

unsafe fn console_write_char(c: u8) {
    if c == b'\n' {
        let row = VGA_INDEX / (VGA_WIDTH * 2);
        VGA_INDEX = (row + 1) * (VGA_WIDTH * 2);
    } else {
        if VGA_INDEX >= VGA_WIDTH * VGA_HEIGHT * 2 {
            VGA_INDEX = 0; // Wrap around for simplicity in this minimal port
        }
        *VGA_BUFFER.add(VGA_INDEX) = c;
        *VGA_BUFFER.add(VGA_INDEX + 1) = 0x07; // Light grey on black
        VGA_INDEX += 2;
    }
}


extern "C" {
    pub fn printk(fmt: *const c_char, ...) -> c_int;
}

// Trampoline to capture variadic arguments.
// For MSVC/Intel syntax, do not use %.
global_asm!(r#"
.global printk
printk:
    push rbp
    mov rbp, rsp
    
    # Save generic registers used for arguments (except RDI which is 'fmt')
    push r9
    push r8
    push rcx
    push rdx
    push rsi
    
    # Now stack at RSP points to standard U64 array: [RSI, RDX, RCX, R8, R9]
    mov rsi, rsp      # Argument 2: args_ptr
    # RDI is already 'fmt'
    
    call rust_printk_impl
    
    add rsp, 40       # Clean up 5 * 8 bytes
    pop rbp
    ret
"#);
