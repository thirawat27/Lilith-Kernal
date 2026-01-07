use crate::types::*;
use crate::string::strlen;
use crate::ctype::isdigit;
use core::ptr;

// Implement VaList for x86_64
// To simplify, `printk` assembly trampoline will dump register arguments onto the stack
// so we can treat it as a contiguous array of 8-byte values.
#[repr(C)]
pub struct VaList {
    ptr: *const u64, // 64-bit stride
}

impl VaList {
    pub unsafe fn from_ptr(ptr: *const c_void) -> Self {
        Self { ptr: ptr as *const u64 }
    }

    pub unsafe fn arg<T>(&mut self) -> T {
        // Assume everything is promoted to 64-bit on the stack (standard for variadic in C to some extent, 
        // especially with our manual push shim).
        // Exceptions would be large structs, but for printk we usually deal with primitives.
        let val = ptr::read(self.ptr as *const T);
        self.ptr = self.ptr.add(1); // Advance by 8 bytes (1 * u64)
        val
    }
}

const ZEROPAD: c_int = 1;
const SIGN: c_int = 2;
const PLUS: c_int = 4;
const SPACE: c_int = 8;
const LEFT: c_int = 16;
const SPECIAL: c_int = 32;
const SMALL: c_int = 64;

fn do_div(n: &mut c_ulong, base: c_ulong) -> c_ulong {
    let res = *n % base;
    *n = *n / base;
    res
}

unsafe fn skip_atoi(s: &mut *const c_char) -> c_int {
    let mut i = 0;
    while isdigit(*(*s) as c_int) != 0 {
        i = i * 10 + (*(*s) as c_int - '0' as c_int);
        *s = (*s).add(1);
    }
    i
}

unsafe fn number(mut str: *mut c_char, mut num: c_long, base: c_int, mut size: c_int, mut precision: c_int, mut type_: c_int) -> *mut c_char {
    let c: c_char;
    let sign: c_char;
    let mut tmp: [c_char; 66] = [0; 66]; // Larger buffer for 64-bit binary/octal
    let digits = if (type_ & SMALL) != 0 {
        b"0123456789abcdefghijklmnopqrstuvwxyz".as_ptr() as *const c_char
    } else {
        b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_ptr() as *const c_char
    };
    let mut i: c_int;

    if (type_ & LEFT) != 0 {
        type_ &= !ZEROPAD;
    }
    if base < 2 || base > 36 {
        return 0 as *mut c_char;
    }
    c = if (type_ & ZEROPAD) != 0 { '0' as c_char } else { ' ' as c_char };
    
    if (type_ & SIGN) != 0 && num < 0 {
        sign = '-' as c_char;
        num = -num;
    } else {
        sign = if (type_ & PLUS) != 0 {
            '+' as c_char
        } else if (type_ & SPACE) != 0 {
            ' ' as c_char
        } else {
            0
        };
    }

    if sign != 0 {
        size -= 1;
    }

    if (type_ & SPECIAL) != 0 {
        if base == 16 {
            size -= 2;
        } else if base == 8 {
            size -= 1;
        }
    }

    i = 0;
    let mut u_num = num as c_ulong;

    if u_num == 0 {
        tmp[i as usize] = '0' as c_char;
        i += 1;
    } else {
        while u_num != 0 {
            tmp[i as usize] = *digits.add(do_div(&mut u_num, base as c_ulong) as usize);
            i += 1;
        }
    }

    if i > precision {
        precision = i;
    }
    size -= precision;

    if (type_ & (ZEROPAD + LEFT)) == 0 {
        while size > 0 {
            *str = ' ' as c_char;
            str = str.add(1);
            size -= 1;
        }
    }

    if sign != 0 {
        *str = sign;
        str = str.add(1);
    }

    if (type_ & SPECIAL) != 0 {
        if base == 8 {
            *str = '0' as c_char;
            str = str.add(1);
        } else if base == 16 {
            *str = '0' as c_char;
            str = str.add(1);
            *str = *digits.add(33); // x or X
            str = str.add(1);
        }
    }

    if (type_ & LEFT) == 0 {
         while size > 0 {
            *str = c;
            str = str.add(1);
            size -= 1;
         }
    }

    while i < precision {
        *str = '0' as c_char;
        str = str.add(1);
        precision -= 1;
    }

    while i > 0 {
        i -= 1;
        *str = tmp[i as usize];
        str = str.add(1);
    }

    while size > 0 {
        *str = ' ' as c_char;
        str = str.add(1);
        size -= 1;
    }

    str
}

#[no_mangle]
pub unsafe extern "C" fn vsprintf(buf: *mut c_char, mut fmt: *const c_char, mut args: VaList) -> c_int {
    let mut len: c_int;
    let mut str = buf;
    let mut s: *mut c_char;
    let mut ip: *mut c_int;
    let mut flags: c_int;
    let mut field_width: c_int;
    let mut precision: c_int;
    let mut _qualifier: c_int;

    while *fmt != 0 {
        if *fmt != '%' as c_char {
            *str = *fmt;
            str = str.add(1);
            fmt = fmt.add(1);
            continue;
        }

        flags = 0;
        loop {
            fmt = fmt.add(1);
            match *fmt as u8 as char {
                '-' => flags |= LEFT,
                '+' => flags |= PLUS,
                ' ' => flags |= SPACE,
                '#' => flags |= SPECIAL,
                '0' => flags |= ZEROPAD,
                _ => break,
            }
        }

        field_width = -1;
        if isdigit(*fmt as c_int) != 0 {
            field_width = skip_atoi(&mut fmt);
        } else if *fmt == '*' as c_char {
            fmt = fmt.add(1);
            field_width = args.arg::<c_int>();
            if field_width < 0 {
                field_width = -field_width;
                flags |= LEFT;
            }
        }

        precision = -1;
        if *fmt == '.' as c_char {
            fmt = fmt.add(1);
            if isdigit(*fmt as c_int) != 0 {
                precision = skip_atoi(&mut fmt);
            } else if *fmt == '*' as c_char {
                 fmt = fmt.add(1);
                 precision = args.arg::<c_int>();
            }
            if precision < 0 {
                precision = 0;
            }
        }

        _qualifier = -1;
        if *fmt == 'h' as c_char || *fmt == 'l' as c_char || *fmt == 'L' as c_char {
            _qualifier = *fmt as c_int;
            fmt = fmt.add(1);
        }

        match *fmt as u8 as char {
            'c' => {
                if (flags & LEFT) == 0 {
                    while field_width > 1 {
                        *str = ' ' as c_char;
                        str = str.add(1);
                        field_width -= 1;
                    }
                }
                *str = args.arg::<c_uchar>() as c_char;
                str = str.add(1);
                while field_width > 1 {
                    *str = ' ' as c_char;
                    str = str.add(1);
                    field_width -= 1;
                }
            },
            's' => {
                let s_arg = args.arg::<*mut c_char>();
                s = if s_arg.is_null() { b"<NULL>\0".as_ptr() as *mut c_char } else { s_arg };
                len = strlen(s) as c_int;
                if precision >= 0 && len > precision {
                    len = precision;
                }
                if (flags & LEFT) == 0 {
                    while len < field_width {
                        *str = ' ' as c_char;
                        str = str.add(1);
                        field_width -= 1;
                    }
                }
                for k in 0..len {
                    *str = *s.add(k as usize);
                    str = str.add(1);
                }
                while len < field_width {
                    *str = ' ' as c_char;
                    str = str.add(1);
                    field_width -= 1;
                }
            },
            'o' => {
                str = number(str, args.arg::<c_ulong>() as c_long, 8, field_width, precision, flags);
            },
            'p' => {
                 if field_width == -1 {
                    field_width = 16; // 64-bit pointers usually hex
                    flags |= ZEROPAD;
                 }
                 str = number(str, args.arg::<c_ulong>() as c_long, 16, field_width, precision, flags);
            },
            'x' => {
                flags |= SMALL;
                str = number(str, args.arg::<c_ulong>() as c_long, 16, field_width, precision, flags);
            },
            'X' => {
                str = number(str, args.arg::<c_ulong>() as c_long, 16, field_width, precision, flags);
            },
            'd' | 'i' => {
                flags |= SIGN;
                str = number(str, args.arg::<c_int>() as c_long, 10, field_width, precision, flags);
            },
            'u' => {
                str = number(str, args.arg::<c_ulong>() as c_long, 10, field_width, precision, flags);
            },
            'n' => {
                ip = args.arg::<*mut c_int>();
                *ip = str.offset_from(buf) as c_int;
            },
            _ => {
                if *fmt != '%' as c_char {
                    *str = '%' as c_char;
                    str = str.add(1);
                }
                if *fmt != 0 {
                    *str = *fmt;
                    str = str.add(1);
                } else {
                    fmt = fmt.sub(1);
                }
            }
        }
        fmt = fmt.add(1); 
    }
    *str = 0;
    str.offset_from(buf) as c_int
}
