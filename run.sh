cargo build --release --target wasm32-wasi --manifest-path driver/Cargo.toml
mv ./target/wasm32-wasi/release/driver.wasm ./driver.wasm
cargo run --release --manifest-path dd-manager/Cargo.toml
