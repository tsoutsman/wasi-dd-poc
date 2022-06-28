#![feature(abi_x86_interrupt)]

use wasmtime::{Engine, Func, Linker, Module, Store};

fn register_handler(num: u32, address: i32) {
    println!("dd-manager/register_handler: {num}, address: {address}");
}

fn panic_handler() {
    loop {}
}

fn main() {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let mut linker = Linker::new(&engine);
    linker
        .define(
            "env",
            "register_handler",
            Func::wrap(&mut store, register_handler),
        )
        .unwrap();

    linker
        .define("env", "handle_panic", Func::wrap(&mut store, panic_handler))
        .unwrap();

    let module = Module::new(&engine, include_bytes!("../../driver.wasm")).unwrap();
    let instance = linker.instantiate(&mut store, &module).unwrap();

    instance
        .get_func(&mut store, "init")
        .unwrap()
        .typed::<u32, (), _>(&mut store)
        .unwrap()
        .call(&mut store, 10)
        .unwrap();
}

#[repr(C)]
pub struct PciDevice {
    vendor_id: u16,
    device_id: u16,
}
