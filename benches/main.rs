use criterion::criterion_main;

mod vec_vs_bytes;
mod hex_vs_faster_hex;
mod regex_compile_vs_non_compile;
criterion_main!{vec_vs_bytes::benches,hex_vs_faster_hex::benches,regex_compile_vs_non_compile::benches}