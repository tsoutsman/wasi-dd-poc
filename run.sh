# cargo build --release --target wasm32-wasi --manifest-path driver/Cargo.toml
# We must cd into the directory for the .cargo/config.toml file to take effect.
cd driver
cargo build --release
cd ..
mv ./target/wasm32-wasi/release/driver.wasm ./driver.wasm
cargo run --release --manifest-path dd-manager/Cargo.toml
