extern crate core;

use std::fmt::Write;

use criterion::{black_box, Criterion, criterion_group, criterion_main};
use ring::digest::Context;
use ring::digest::SHA256;

pub fn gen_using_faster_hex(v: &[impl AsRef<[u8]>]) -> String {
    let mut ctx = Context::new(&SHA256);
    for src in v {
        ctx.update(src.as_ref());
    }
    faster_hex::hex_string(ctx.finish().as_ref())
}
pub fn gen_using_hex(v: &[impl AsRef<[u8]>]) -> String {
    let mut ctx = Context::new(&SHA256);
    for src in v {
        ctx.update(src.as_ref());
    }
    hex::encode(ctx.finish().as_ref())
}

pub fn gen_using_string(v: &[impl AsRef<[u8]>]) -> String {
    let mut ctx = Context::new(&SHA256);
    for src in v {
        ctx.update(src.as_ref());
    }
    let digest = ctx.finish();
    let mut hash_str = String::with_capacity(64);
    digest
        .as_ref()
        .iter()
        .for_each(|byte| write!(hash_str, "{byte:02x}")
            .expect("write! macro on string cannot fail"));

    hash_str
}
pub fn gen_using_string_concate(v: &[impl AsRef<[u8]>]) -> String{
    let mut ctx = Context::new(&SHA256);
    for src in v {
        ctx.update(src.as_ref());
    }
    let digest = ctx.finish();
    let out: Vec<String> = digest
        .as_ref()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    out.join("")
}

fn bench_comparison(c: &mut Criterion) {
    let input = vec![b"hello", b"world", b"hello", b"hello"];
    let mut group = c.benchmark_group("gen comparison");
    group.bench_function("existing string concatenate", |b| b.iter(|| {
        gen_using_string_concate(&black_box(&input))
    }));
    group.bench_function("using write macro", |b| b.iter(|| {
        gen_using_string(&black_box(&input))
    }));
    group.bench_function("using hex", |b| b.iter(|| {
        gen_using_hex(&black_box(&input))
    }));
    group.bench_function("using faster-hex", |b| b.iter(|| {
        gen_using_faster_hex(&black_box(&input))
    }));
    group.finish();
}
criterion_group!(benches, bench_comparison);
criterion_main!(benches);