mod world_cities;
pub use world_cities::{
    bulkload::world_cities_benchmark as world_cities_bulkload_benchmark,
    sequential::world_cities_benchmark as world_cities_sequential_benchmark,
};
