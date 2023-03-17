# Benchmark
This module benchmarks Rust implementations of multidimensional indexing structures using uniform (random) and clustered (real-world) datasets. 

## Software
The following list of indexing trees were benchmarked for build and query performance: 
| Index      | Code                                                             | Sequential build    | Bulk-loading  | Runtime dimensions  |
| :---       |     :---                                                         | :---:               | :---:         | :---:    |
| Ball-tree  | [petal-neighbors](https://github.com/petabi/petal-neighbors)     | &cross;             | &check;       | &check;  |
| R-tree     | [rtree](https://github.com/tidwall/rtree.rs)                     | &check;             | &cross;       | &cross;  |
| R*-tree    | [rstar](https://github.com/georust/rstar)                        | &check;             | &check;       | &cross;  |
| SR-tree.   | [srtree](https://github.com/aicers/srtree)                       | &check;             | &check;       | &check;  |

In this context, constructing the tree by inserting points one by one is referred to as sequential build, 
while packing and loading points to build the tree is known as bulk-loading. Additionally, 
if the dimension of the tree can be set during program execution, it is referred to as runtime dimensions, 
as opposed to compile-time dimensions which are set during compilation ([const-generics](https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html#what-are-const-generics)).


## Benchmark
A benchmark consists of two rounds: `build` to construct the tree (sequentially or bulk-loading) and `query` to find k nearest neighbors for each point in the tree.
Common real-world scenarios (e.g. clustering) need performance in both. 

## Usage

## Performance
