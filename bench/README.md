# Benchmark
This module benchmarks Rust implementations of multidimensional indexing structures using uniform (random) and clustered (real-world) datasets. 

## Software
The following list of indexing trees were benchmarked for build and query performance: 
| Index      | Code                                                             | Sequential build    | Bulk-loading  | Runtime dimensions  |
| :---       |     :---                                                         | :---:               | :---:         | :---:    |
| Ball-tree  | [petal-neighbors](https://github.com/petabi/petal-neighbors)     | &cross;             | &check;       | &check;  |
| R-tree     | [rtree](https://github.com/tidwall/rtree.rs)                     | &check;             | &cross;       | &cross;  |
| R*-tree    | [rstar](https://github.com/georust/rstar)                        | &check;             | &check;       | &cross;  |
| SR-tree.   | [srtree](https://github.com/aicers/srtree)                       | &cross;             | &check;       | &check;  |

- Sequential build - constructing the tree by inserting points one by one
- Bulk-loading - packing and loading points at once to build the tree
- Runtime dimensions - the dimension of the tree can be set during program execution (point coordinates are represented using `Vector`s), as opposed to compile-time dimensions which are set during compilation ([const-generics](https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html#what-are-const-generics)).


## Benchmark
A benchmark round consists of two tasks: `build` to construct the tree (sequentially or bulk-loading) and `query` to find k nearest neighbors for each point in the tree.

## Requirements
[R*-tree](https://github.com/georust/rstar) supports maximum 9-dimensions. To run the benchmarks at high dimensions, R*-tree should [implement point for arrays](https://github.com/georust/rstar/blob/27f74beaf2a79dff11fd4e7f1c6fc97f8b54b367/rstar/src/point.rs#L348), for example (16-dimensions):
```
implement_point_for_array!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
```

## Performance
We measured the performance of the indexing structures on MBP 14" (2021, M1 Pro, charger plugged in). These results were obtained [Criterion](https://github.com/bheisler/criterion.rs)'s best estimate of execution time from 100 benchmark rounds.

### Uniform dataset
 2000 points (at varying dimensions) were randomly generated with the same seed:

| Dimension      | [R*-tree](https://github.com/georust/rstar) | [Ball-tree](https://github.com/petabi/petal-neighbors) | [SR-tree](https://github.com/aicers/srtree)       | Linear scan    |
| :---           | :---                                         | :---                                                   | :---                                         | :---      |
| 2              | 4.2677 ms                                    | 84.227 ms                                              | 7.0148 ms                                    | 115.93 ms |
| 4              | 11.862 ms                                    | 100.26 ms                                              | 16.198 ms                                    | 114.90 ms |
| 8              | 42.211 ms                                    | 133.91 ms                                              | 34.549 ms                                    | 117.22 ms |
| 16             | 187.53 ms                                    | 192.28 ms                                              | 59.724 ms                                    | 123.39 ms |
| 32             | 250.49 ms                                    | 316.91 ms                                              | 101.04 ms                                    | 145.02 ms |
| 64             | 455.50 ms                                    | 640.49 ms                                              | 201.19 ms                                    | 170.40 ms |
| 96             | 667.66 ms                                    | 957.28 ms                                              | 303.21 ms                                    | 237.64 ms |
| 124            | 857.98 ms                                    | 1.2197 s                                               | 427.39 ms                                    | 310.74 ms |
| 200            | -                                            | 2.0785 s                                               | 680.21 ms                                    | 485.11 ms |
| 300            | -                                            | 3.2570 s                                               | 1.0893 s                                     | 813.61 ms |

- Linear scan - a brute force solution to query k nearest neighbors by linearly scanning all points.    
- R*-tree can't be compiled for dimensions >= 128. 
