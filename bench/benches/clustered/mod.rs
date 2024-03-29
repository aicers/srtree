mod base;
pub mod benchmark;
mod data;
pub use base::benchmark as srtree_clustered_benchmark;
pub use benchmark::build_and_query as clustered_benchmark;
