use criterion::{criterion_group, criterion_main};
mod uniform;
use uniform::srtree_uniform_benchmark;
use uniform::uniform_benchmark;
mod clustered;
mod neighbor;
use clustered::clustered_benchmark;
use clustered::srtree_clustered_benchmark;

criterion_group!(benches, srtree_clustered_benchmark);
criterion_main!(benches);
