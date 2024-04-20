use criterion::{black_box, Criterion, criterion_group, criterion_main};
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use regex::Regex;
lazy_static! {
    static ref REGEX: Regex =
        Regex::new(
            r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)+$",
        ).unwrap();
}
struct EmailValidatorRegexCompiledEachTime {}

struct EmailValidatorRegexCompiledOnce {
}
impl EmailValidatorRegexCompiledEachTime{
    pub fn validate(&self, email: &str) -> bool {
        // let p = Regex::new(
        //     r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)+$",
        // ).unwrap();
        static p: Lazy<Regex> = Lazy::new(|| {
             Regex::new(
                r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)+$",
            ).unwrap()
        });
        p.is_match(email)
    }
}

impl EmailValidatorRegexCompiledOnce {
    pub fn validate(&self, email: &str) -> bool {
        REGEX.is_match(email)
    }
}

fn bench_comparison(c: &mut Criterion) {
    let email =
        "8634323v@jdkfjadhfoaij.com";
    // let input = vec![b"hello", b"world", b"hello", b"hello"];
    let reg_each = EmailValidatorRegexCompiledEachTime{};
    let reg_once=EmailValidatorRegexCompiledOnce{
    };
    REGEX.is_match("email");
    let mut group = c.benchmark_group("regex compilation impact");
    group.bench_function("regex compiled each time", |b| b.iter(|| {
      reg_each.validate(&black_box(email))
    }));
    group.bench_function("regex compiled once", |b| b.iter(|| {
        reg_once.validate(&black_box(email))
    }));

    group.finish();
}
criterion_group!(benches, bench_comparison);
criterion_main!(benches);