#![no_std]

use driver_common;

#[no_mangle]
pub extern "C" fn init(_dev: u32) {
    driver_common::register_handler(3, handler);
}

extern "C" fn handler() {
    todo!()
}
