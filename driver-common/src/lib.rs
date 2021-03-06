#![no_std]

mod ffi {
    extern "C" {
        // TODO: Is this sound? The type signature in dd-manager is fn(). (no -> !)
        pub fn handle_panic() -> !;
        pub fn register_handler(num: u32, func: extern "C" fn());
    }
}

pub fn register_handler(num: u32, handler: extern "C" fn()) {
    unsafe { ffi::register_handler(num, handler) }
}

#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    unsafe { ffi::handle_panic() };
}
