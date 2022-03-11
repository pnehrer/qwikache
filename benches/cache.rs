use criterion::{black_box, criterion_group, criterion_main, Criterion};
use qwikache::Cache;
use std::time::{Duration, Instant};

fn get_expired(c: &mut Criterion) {
    let mut cache = Cache::default();
    let now = Instant::now();
    for i in 0..10_000_000 {
        let exp = now + Duration::from_secs(1000 + i % 1000);
        cache.put_exp(format!("test_key_{}", i), "test_value", Some(exp));
    }

    let mut i = 0;
    c.bench_function("get_expired", |b| {
        b.iter(|| {
            let cached = cache.get(&format!("test_key_{}", i));
            i += 1;
            black_box(cached.is_some());
        })
    });
}

fn get_unexpired(c: &mut Criterion) {
    let mut cache = Cache::default();
    for i in 0..10_000_000 {
        cache.put(format!("test_key_{}", i), "test_value");
    }

    let mut i = 0;
    c.bench_function("get_unexpired", |b| {
        b.iter(|| {
            let cached = cache.get(&format!("test_key_{}", i));
            i += 1;
            black_box(cached.is_some());
        })
    });
}

criterion_group!(benches, get_expired, get_unexpired);
criterion_main!(benches);
