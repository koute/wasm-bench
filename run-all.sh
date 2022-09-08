#!/bin/sh

rm -f results.csv

BENCHES="bench_regexredux,bench_coremark,bench_noop"

cargo build -p kernel_noop --profile lto --target=wasm32-unknown-unknown
cargo build -p kernel_regexredux --profile lto --target=wasm32-unknown-unknown

cargo build -p kernel_wasmi --profile lto-wasmi_016 --no-default-features --features wasmi_016 --target=wasm32-unknown-unknown
cargo build -p kernel_wasmi --profile lto-wasmi_013 --no-default-features --features wasmi_013 --target=wasm32-unknown-unknown
cargo build -p kernel_wasmi --profile lto-wasmi_009 --no-default-features --features wasmi_009 --target=wasm32-unknown-unknown

cargo run --release --features target_wasmi_under_wasmtime,wasmtime_040,wasmi_016,$BENCHES | tee -a results.csv
cargo run --release --features target_wasmi_under_wasmtime,wasmtime_040,wasmi_013,$BENCHES | tee -a results.csv
cargo run --release --features target_wasmi_under_wasmtime,wasmtime_040,wasmi_009,$BENCHES | tee -a results.csv
cargo run --release --features target_wasmi_under_wasmtime,wasmtime_038,wasmi_016,$BENCHES | tee -a results.csv
cargo run --release --features target_wasmi_under_wasmtime,wasmtime_038,wasmi_013,$BENCHES | tee -a results.csv
cargo run --release --features target_wasmi_under_wasmtime,wasmtime_038,wasmi_009,$BENCHES | tee -a results.csv

cargo run --profile lto --features target_wasmi_on_bare_metal,wasmi_016,$BENCHES | tee -a results.csv
cargo run --profile lto --features target_wasmi_on_bare_metal,wasmi_013,$BENCHES | tee -a results.csv
cargo run --profile lto --features target_wasmi_on_bare_metal,wasmi_009,$BENCHES | tee -a results.csv

cargo run --release --features target_wasmtime,wasmtime_040,$BENCHES | tee -a results.csv
cargo run --release --features target_wasmtime,wasmtime_038,$BENCHES | tee -a results.csv

cargo run --profile lto --features target_bare_metal,$BENCHES | tee -a results.csv
