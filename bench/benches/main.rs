use criterion::{criterion_group, criterion_main};
mod uniform;
use uniform::uniform_combined_benchmark;

criterion_group!(benches, uniform_combined_benchmark);
criterion_main!(benches);
