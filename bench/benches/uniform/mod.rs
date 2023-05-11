mod base;
mod benchmark;
mod benchmark_query_radius;
mod data;

pub use base::benchmark as srtree_uniform_benchmark;
pub use benchmark::build_and_query as uniform_benchmark;
pub use benchmark_query_radius::build_and_query as uniform_benchmark_query_radius;
