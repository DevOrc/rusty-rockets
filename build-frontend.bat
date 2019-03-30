@cd frontend
cargo build --target wasm32-unknown-unknown
@echo Building website!
wasm-pack build --out-dir ../server/assets --no-typescript --target no-modules
@cd..