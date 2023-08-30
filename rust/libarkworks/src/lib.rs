use std::slice;

use libc::c_uchar;

mod bn254;
mod utils;
mod serialize;

#[no_mangle]
pub extern "system" fn libarkworks_g1_is_valid(
    x: *const [c_uchar; 32],
    y: *const [c_uchar; 32],
) -> bool {
    let x = unsafe { &(*x) };
    let y = unsafe { &(*y) };
    bn254::g1_is_valid([&x[..], y].concat())
}

#[no_mangle]
pub extern "system" fn libarkworks_g2_is_valid(
    a: *const [c_uchar; 32],
    b: *const [c_uchar; 32],
    c: *const [c_uchar; 32],
    d: *const [c_uchar; 32],
) -> bool {
    let a = unsafe { &(*a) };
    let b = unsafe { &(*b) };
    let c = unsafe { &(*c) };
    let d = unsafe { &(*d) };
    bn254::g2_is_valid([&a[..], b, c, d].concat())
}

#[no_mangle]
pub extern "system" fn libarkworks_add_g1(
    a: *const [c_uchar; 64],
    b: *const [c_uchar; 64],
    result: *mut [c_uchar; 64],
) -> bool {
    let a_serialized = unsafe { &(*a) };
    let b_serialized = unsafe { &(*b) };
    let result = unsafe { &mut *result };
    result.copy_from_slice(&bn254::add_g1(a_serialized, b_serialized));
    // TODO: add errors handling
    return true;
}

#[no_mangle]
pub extern "system" fn libarkworks_mul_g1(
    p: *const [c_uchar; 64],
    s: *const [c_uchar; 32],
    result: *mut [c_uchar; 64],
) -> bool {
    let p_serialized = unsafe { &(*p) };
    let s_serialized = unsafe { &(*s) };
    let result = unsafe { &mut *result };
    result.copy_from_slice(&bn254::mul_g1(p_serialized, s_serialized));
    // TODO: add errors handling
    return true;
}

#[no_mangle]
pub extern "system" fn libarkworks_pairing_check(
    g1s: *const c_uchar, // fix it
    g2s: *const c_uchar,
    pairs: u32,
) -> bool {
    let g1s_serialized = unsafe { slice::from_raw_parts(g1s, 64 * pairs as usize) };
    let g2s_serialized = unsafe { slice::from_raw_parts(g2s, 128 * pairs as usize) };
    bn254::pairing_check(g1s_serialized, g2s_serialized, pairs as usize)
}

#[no_mangle]
pub extern "system" fn libarkworks_random_g1(
    result: *mut [c_uchar; 64],
) {
    let result = unsafe { &mut *result };
    result.copy_from_slice(&bn254::random_g1());
}

#[no_mangle]
pub extern "system" fn libarkworks_random_g2(
    result: *mut [c_uchar; 128],
) {
    let result = unsafe { &mut *result };
    result.copy_from_slice(&bn254::random_g2());
}