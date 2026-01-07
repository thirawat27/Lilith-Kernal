#![allow(non_camel_case_types)]

// Basic C types for x86_64 (64-bit)

pub type c_int = i32;
pub type c_char = i8;
pub type c_uchar = u8;
pub type c_long = i64; 
pub type c_ulong = u64;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_void = core::ffi::c_void;
pub type size_t = usize;
pub type ssize_t = isize;
