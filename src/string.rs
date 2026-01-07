use crate::types::*;
use core::ptr;

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void {
    ptr::copy_nonoverlapping(src as *const u8, dest as *mut u8, n);
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void {
    ptr::copy(src as *const u8, dest as *mut u8, n);
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void {
    ptr::write_bytes(s as *mut u8, c as u8, n);
    s
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(cs: *const c_void, ct: *const c_void, count: size_t) -> c_int {
    let s1 = core::slice::from_raw_parts(cs as *const u8, count);
    let s2 = core::slice::from_raw_parts(ct as *const u8, count);
    // Standard memcmp compares bytes unsigned
    for i in 0..count {
        if s1[i] != s2[i] {
            return (s1[i] as c_int) - (s2[i] as c_int);
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const c_char) -> size_t {
    let mut len = 0;
    while *s.add(len) != 0 {
        len += 1;
    }
    len
}

#[no_mangle]
pub unsafe extern "C" fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char {
    let mut i = 0;
    loop {
        let c = *src.add(i);
        *dest.add(i) = c;
        if c == 0 { break; }
        i += 1;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char {
    let mut i = 0;
    while i < n {
        let c = *src.add(i);
        *dest.add(i) = c;
        if c == 0 {
            // pad with zeros
            for j in (i + 1)..n {
                *dest.add(j) = 0;
            }
            break; 
        }
        i += 1;
    }
    // If we hit n, no null termination is added, which is correct for strncpy
    dest
}

#[no_mangle]
pub unsafe extern "C" fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int {
    let mut i = 0;
    loop {
        let c1 = *cs.add(i) as c_uchar;
        let c2 = *ct.add(i) as c_uchar;
        if c1 != c2 {
            return (c1 as c_int) - (c2 as c_int);
        }
        if c1 == 0 {
            return 0;
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn strncmp(cs: *const c_char, ct: *const c_char, count: size_t) -> c_int {
    let mut i = 0;
    while i < count {
        let c1 = *cs.add(i) as c_uchar;
        let c2 = *ct.add(i) as c_uchar;
        if c1 != c2 {
            return (c1 as c_int) - (c2 as c_int);
        }
        if c1 == 0 {
            return 0;
        }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char {
    let mut i = 0;
    while *dest.add(i) != 0 {
        i += 1;
    }
    let mut j = 0;
    loop {
        let c = *src.add(j);
        *dest.add(i + j) = c;
        if c == 0 { break; }
        j += 1;
    }
    dest
}
