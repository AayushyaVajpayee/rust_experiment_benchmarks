use criterion::{black_box, Criterion, criterion_group, criterion_main};

enum Di{
    RingDigest(ring::digest::Digest)
}
impl AsRef<[u8]> for Di{
    fn as_ref(&self) -> &[u8] {
       match self { Di::RingDigest(a) => {a.as_ref()} }
    }
}
fn generate_digest_vec(message: &[u8]) -> Vec<u8> {
    let digest = ring::digest::digest(&ring::digest::SHA256, message);
    digest.as_ref().to_vec()
}

fn generate_digest_non_vec(message: &[u8]) -> Di {
    let digest = ring::digest::digest(&ring::digest::SHA256, message);
    Di::RingDigest(digest)
}

fn verify_signature_non_vec(
    signature: &[u8],
    msg: &[u8],
) -> bool {
    let hashed_digest = generate_digest_non_vec(msg);
    let hashed_digest_into_bytes = hashed_digest.as_ref();
    hashed_digest_into_bytes == signature
}

fn verify_signature_vec(
    signature: &[u8],
    msg: &[u8],
) -> bool {
    let hashed_digest = generate_digest_vec(msg);
    let hashed_digest_into_bytes = hashed_digest.as_slice();
    hashed_digest_into_bytes == signature
}

pub fn bench_comparison(c: &mut Criterion) {
    let right_signature =
        hex::decode("123250a72f4e961f31661dbcee0fec0f4714715dc5ae1b573f908a0a5381ddba").unwrap();
    let wrong_signature =
        hex::decode("123250a72f4e961f31661dbcee0fec0f4714715dc5ae1b573f908a0a5381ddbb")
            .expect("Wrong signature decoding");
    // let input = vec![b"hello", b"world", b"hello", b"hello"];
    let mut group = c.benchmark_group("cloning bytes");
    group.bench_function("converting to vector", |b| b.iter(|| {
        verify_signature_vec(&black_box(&right_signature.as_slice()),&black_box(wrong_signature.as_slice()))
    }));
    group.bench_function("not converting to vector", |b| b.iter(|| {
        verify_signature_non_vec(&black_box(&right_signature.as_slice()),&black_box(wrong_signature.as_slice()))
    }));
    
    group.finish();
}
criterion_group!(benches, bench_comparison);
criterion_main!(benches);