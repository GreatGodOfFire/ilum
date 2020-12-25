#![no_builtins]
#![no_main]
#![no_std]

use ilum_init;

#[no_mangle]
pub extern "C" fn main() {
    unsafe {
        enable();
        led();
    }

    loop {}
}

pub unsafe fn led() {
    core::ptr::write_volatile(0x4200_4084 as *mut u32, 1 << 3);
}

pub unsafe fn enable() {
    let reg = 0x400A_C068 as *mut u32;
    core::ptr::write_volatile(reg, 0xFFFF_FFFF);

    let reg = 0x400A_C06C as *mut u32;
    core::ptr::write_volatile(reg, 0xFFFF_FFFF);

    let reg = 0x4200_0004 as *mut u32;
    core::ptr::write_volatile(reg, 0xFFFF_FFFF);

    let reg = 0x4200_4004 as *mut u32;
    core::ptr::write_volatile(reg, 0xFFFF_FFFF);
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
