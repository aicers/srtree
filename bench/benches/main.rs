use criterion::{criterion_group, criterion_main};
mod clustered;
use clustered::clustered_benchmark;
use clustered::srtree_clustered_benchmark;
mod neighbor;
mod uniform;
use uniform::srtree_uniform_benchmark;
use uniform::uniform_benchmark;

criterion_group!(benches, uniform_benchmark);
criterion_main!(benches);
