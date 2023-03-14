use criterion::{criterion_group, criterion_main};
mod clustered;
use clustered::world_cities_sequential_benchmark;
use clustered::world_cities_bulkload_benchmark;
mod uniform;
use uniform::uniform_sequential_benchmark;
use uniform::uniform_bulkload_benchmark;

criterion_group!(benches, uniform_sequential_benchmark);
criterion_main!(benches);
