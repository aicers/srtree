use criterion::{criterion_group, criterion_main};
mod clustered;
#[allow(unused)]
use clustered::{clustered_benchmark, srtree_clustered_benchmark};
mod uniform;
#[allow(unused)]
use uniform::{srtree_uniform_benchmark, uniform_benchmark, uniform_benchmark_query_radius};
mod utils;

criterion_group!(benches, srtree_uniform_benchmark);
criterion_main!(benches);
