#![feature(abi_x86_interrupt)]

use std::cell::RefCell;

use wasmtime::{Engine, Func, Linker, Module, Store, Val};

thread_local! {
    static HANDLERS: RefCell<Vec<(u8, i32)>> = RefCell::new(Vec::new());
}

fn register_handler(num: u32, address: i32) {
    println!("dd-manager/register_handler: {num}, address: {address}");
    // TODO: Do we really need thread locals for this?
    HANDLERS.with(|rc| rc.borrow_mut().push((num as u8, address)))
}

fn panic_handler() {
    // TODO: Take PanicInfo and log error or smth.
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

    let function_table = instance
        .get_table(&mut store, "__indirect_function_table")
        .unwrap();

    let handlers: Vec<_> = HANDLERS
        .with(|rc| rc.take())
        .into_iter()
        .map(|(num, address)| {
            let function = match function_table.get(&mut store, address as u32).unwrap() {
                Val::FuncRef(f) => f,
                _ => panic!(),
            };
            (num, function)
        })
        .collect();

    // TODO: Separate handlers from store?
    println!("handlers: {:#?}", handlers);
}
