
use super::u::*;
use std::mem::transmute;

pub fn f32_to_bytes(x: f32) -> [u8; 4]
{
    u32_to_bytes(unsafe { transmute(x) })
}

pub fn f64_to_bytes(x: f64) -> [u8; 8]
{
    u64_to_bytes(unsafe { transmute(x) })
}

pub fn bytes_to_f32(x: &[u8; 4]) -> f32
{
    unsafe { transmute(bytes_to_u32(&x)) }
}

pub fn bytes_to_f64(x: &[u8; 8]) -> f64
{
    unsafe { transmute(bytes_to_u64(&x)) }
}