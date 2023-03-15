use criterion::{criterion_group, criterion_main};
mod uniform;
use uniform::uniform_bulkload_benchmark;
use uniform::uniform_combined_benchmark;
use uniform::uniform_sequential_benchmark;

criterion_group!(benches, uniform_combined_benchmark);
criterion_main!(benches);
