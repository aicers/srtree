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

## Requirements
[R*-tree](https://github.com/georust/rstar) supports maximum 9-dimensions. To run the benchmarks at high dimensions, R*-tree should [implement point for arrays](https://github.com/georust/rstar/blob/27f74beaf2a79dff11fd4e7f1c6fc97f8b54b367/rstar/src/point.rs#L348), for example (16-dimensions):
```
implement_point_for_array!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
```

## Performance
We measured the performance of the indexing structures on MBP 14" (2021, M1 Pro). Uniform datasets (at varying dimensions) were randomly generated with the same seed. These results were obtained [Criterion](https://github.com/bheisler/criterion.rs)'s best estimate of execution time from 100 benchmark rounds:

| Dimension      | [R*-tree](https://github.com/georust/rstar) | [Ball-tree](https://github.com/petabi/petal-neighbors) | [SR-tree](https://github.com/aicers/srtree)       | Linear scan    |
| :---           | :---                                        | :---                                                   | :---          | :---      |
| 2              | **4.2677 ms**                               | 84.227 ms                                              | 8.0524 ms     | 191.60 ms |
| 4              | **11.862 ms**                               | 100.26 ms                                              | 19.597 ms     | 193.33 ms |
| 8              | **42.211 ms**                               | 133.91 ms                                              | 77.242 ms     | 198.53 ms |
| 16             | 187.53 ms                                   | 192.28 ms                                              | **157.77 ms** | 210.66 ms |
| 32             | 250.49 ms                                   | 316.91 ms                                              | **187.94 ms** | 232.78 ms |
| 64             | 455.50 ms                                   | 640.49 ms                                              | **266.66 ms** | 314.63 ms |
| 96             | 667.66 ms                                   | 957.28 ms                                              | **369.25 ms** | 433.5 ms  |
| 124            | 857.98 ms                                   | 1.2197 s                                               | **468.42 ms** | 536.31 ms |
| 200            | -                                           | 2.0785 s                                               | **716.91 ms** | 816.75 ms |
| 300            | -                                           | 3.2570 s                                               | 1.2011 s      | **1.1899 s** |

- Linear scan - a brute force solution to query k nearest neighbors by linearly scanning all points.    
- R*-tree can't be compiled for dimensions >= 128. 
 
