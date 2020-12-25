#![no_builtins]
#![no_std]

mod bootdata;

extern "C" {
    fn main();
}

#[link_section = ".startup"]
#[no_mangle]
pub unsafe extern "C" fn _start() {
    main();
}
