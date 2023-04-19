pub mod benchmark;
mod utils;
mod base;
pub use benchmark::build_and_query as clustered_benchmark;
pub use base::benchmark as srtree_clustered_benchmark;
