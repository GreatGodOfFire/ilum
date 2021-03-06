#![allow(clippy::transmute_ptr_to_ptr)]

use crate::_start;
use core::mem::transmute;

extern "C" {
    fn _boot_stack();
    static _image_length: core::ffi::c_void;
}

#[link_section = ".vectors"]
#[no_mangle]
pub static _VECTOR_TABLE: [unsafe extern "C" fn(); 2] = [_boot_stack, _start];

#[link_section = ".bootdata"]
#[no_mangle]
pub static _BOOT_DATA: [Option<&'static ()>; 3] =
    [None, unsafe { Some(transmute(&_image_length)) }, None];

#[link_section = ".ivt"]
#[no_mangle]
pub static _IMAGE_VECTOR_TABLE: [Option<&'static ()>; 8] = [
    unsafe { Some(transmute(0x4020_00D1)) },
    unsafe { Some(transmute(&_VECTOR_TABLE)) },
    None,
    None,
    unsafe { Some(transmute(&_BOOT_DATA)) },
    unsafe { Some(transmute(&_IMAGE_VECTOR_TABLE)) },
    None,
    None,
];

#[link_section = ".flashconfig"]
#[no_mangle]
pub static _FLEXSPI_NOR_CONFIG: [u32; 128] = [
    0x4246_4346,
    0x5601_0000,
    0,
    0x0002_0101,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0x0003_0401,
    0,
    0,
    0x0020_0000,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0x0A18_04EB,
    0x2604_3206,
    0,
    0,
    0x2404_0405,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0x0000_0406,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0x0818_0420,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0x0818_04D8,
    0,
    0,
    0,
    0x0818_0402,
    0x0000_2004,
    0,
    0,
    0,
    0,
    0,
    0,
    0x0000_0460,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    256,
    4096,
    1,
    0,
    0x0001_0000,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
