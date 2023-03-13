use criterion::{criterion_group, criterion_main};
mod clustered;
use clustered::world_cities_sequential_benchmark;

criterion_group!(benches, world_cities_sequential_benchmark);
criterion_main!(benches);
