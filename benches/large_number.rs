use rand_key::RandKey;
use criterion::{criterion_group, criterion_main, Criterion};




fn init_randkey(number: (&str, &str, &str)) -> Result<(), Box<dyn std::error::Error>> {
    let r_p = RandKey::new(number.0, number.1, number.2)?;
    r_p.join()?;
    Ok(())
}


pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("RandKey: 1000000 1000000 1000000", |b| b.iter(|| init_randkey(("1000000", "1000000", "1000000"))));
    c.bench_function("RandKey: 100000000 0 0", |b| b.iter(|| init_randkey(("1000000", "1000000", "1000000"))));
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
