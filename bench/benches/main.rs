use criterion::{criterion_group, criterion_main};
mod uniform;
use uniform::srtree_uniform_benchmark;
use uniform::uniform_benchmark;
mod neighbor;

criterion_group!(benches, uniform_benchmark);
criterion_main!(benches);
