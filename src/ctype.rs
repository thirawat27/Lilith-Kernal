use crate::types::*;

pub const _U: u8 = 0x01;  /* upper */
pub const _L: u8 = 0x02;  /* lower */
pub const _D: u8 = 0x04;  /* digit */
pub const _C: u8 = 0x08;  /* cntrl */
pub const _P: u8 = 0x10;  /* punct */
pub const _S: u8 = 0x20;  /* white space (space/lf/tab) */
pub const _X: u8 = 0x40;  /* hex digit */
pub const _SP: u8 = 0x80; /* hard space (0x20) */

#[no_mangle]
pub static _ctype: [u8; 257] = [
    0x00,			/* EOF */
    _C,_C,_C,_C,_C,_C,_C,_C,			/* 0-7 */
    _C,_C|_S,_C|_S,_C|_S,_C|_S,_C|_S,_C,_C,		/* 8-15 */
    _C,_C,_C,_C,_C,_C,_C,_C,			/* 16-23 */
    _C,_C,_C,_C,_C,_C,_C,_C,            /* 24-31 */
    _S|_SP,_P,_P,_P,_P,_P,_P,_P,		/* 32-39 */
    _P,_P,_P,_P,_P,_P,_P,_P,			/* 40-47 */
    _D,_D,_D,_D,_D,_D,_D,_D,			/* 48-55 */
    _D,_D,_P,_P,_P,_P,_P,_P,			/* 56-63 */
    _P,_U|_X,_U|_X,_U|_X,_U|_X,_U|_X,_U|_X,_U,	/* 64-71 */
    _U,_U,_U,_U,_U,_U,_U,_U,			/* 72-79 */
    _U,_U,_U,_U,_U,_U,_U,_U,			/* 80-87 */
    _U,_U,_U,_P,_P,_P,_P,_P,			/* 88-95 */
    _P,_L|_X,_L|_X,_L|_X,_L|_X,_L|_X,_L|_X,_L,	/* 96-103 */
    _L,_L,_L,_L,_L,_L,_L,_L,			/* 104-111 */
    _L,_L,_L,_L,_L,_L,_L,_L,			/* 112-119 */
    _L,_L,_L,_P,_P,_P,_P,_C,			/* 120-127 */
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,		/* 128-143 */
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,		/* 144-159 */
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,		/* 160-175 */
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,		/* 176-191 */
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,		/* 192-207 */
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,		/* 208-223 */
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,		/* 224-239 */
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0         /* 240-255 */
];

#[no_mangle]
pub extern "C" fn isalnum(c: c_int) -> c_int {
    let idx = (c + 1) as usize;
    if idx < 257 {
        (_ctype[idx] & (_U | _L | _D)) as c_int
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn isalpha(c: c_int) -> c_int {
    let idx = (c + 1) as usize;
    if idx < 257 {
        (_ctype[idx] & (_U | _L)) as c_int
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn iscntrl(c: c_int) -> c_int {
    let idx = (c + 1) as usize;
    if idx < 257 {
        (_ctype[idx] & (_C)) as c_int
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn isdigit(c: c_int) -> c_int {
    let idx = (c + 1) as usize;
    if idx < 257 {
        (_ctype[idx] & (_D)) as c_int
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn isgraph(c: c_int) -> c_int {
    let idx = (c + 1) as usize;
    if idx < 257 {
        (_ctype[idx] & (_P | _U | _L | _D)) as c_int
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn islower(c: c_int) -> c_int {
    let idx = (c + 1) as usize;
    if idx < 257 {
        (_ctype[idx] & (_L)) as c_int
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn isprint(c: c_int) -> c_int {
    let idx = (c + 1) as usize;
    if idx < 257 {
        (_ctype[idx] & (_P | _U | _L | _D | _SP)) as c_int
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn ispunct(c: c_int) -> c_int {
    let idx = (c + 1) as usize;
    if idx < 257 {
        (_ctype[idx] & (_P)) as c_int
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn isspace(c: c_int) -> c_int {
    let idx = (c + 1) as usize;
    if idx < 257 {
        (_ctype[idx] & (_S)) as c_int
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn isupper(c: c_int) -> c_int {
    let idx = (c + 1) as usize;
    if idx < 257 {
        (_ctype[idx] & (_U)) as c_int
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn isxdigit(c: c_int) -> c_int {
    let idx = (c + 1) as usize;
    if idx < 257 {
        (_ctype[idx] & (_D | _X)) as c_int
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn isascii(c: c_int) -> c_int {
    if (c as u32) <= 0x7f { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn toascii(c: c_int) -> c_int {
    (c as u32 & 0x7f) as c_int
}

#[no_mangle]
pub extern "C" fn tolower(c: c_int) -> c_int {
    if isupper(c) != 0 {
        c + 32
    } else {
        c
    }
}

#[no_mangle]
pub extern "C" fn toupper(c: c_int) -> c_int {
    if islower(c) != 0 {
        c - 32
    } else {
        c
    }
}
