[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/aicers/srtree/ci.yml?style=plastic)](#)
[![crates.io](https://img.shields.io/crates/v/srtree)](https://crates.io/crates/srtree)

# srtree
Rust implementation of [SR-Tree](https://dl.acm.org/doi/10.1145/253262.253347): nearest neighbor search index for high-dimensional clustered datasets.

## Example
This example shows how to query nearest neighbors:
```rust
use srtree::SRTree;

fn main() {
    let points = vec![
        vec![0., 0.],
        vec![1., 1.],
        vec![2., 2.],
        vec![3., 3.],
        vec![4., 4.],
    ];
    let tree = SRTree::euclidean(&points).expect("Failed to build SRTree");

    let (indices, distances) = tree.query(&[8., 8.], 3);
    println!("{indices:?}"); // [4, 3, 2] (sorted by distance)
    println!("{distances:?}");
}
```

## License

Copyright 2019-2023 EINSIS, Inc.

Licensed under [Apache License, Version 2.0][apache-license] (the "License");
you may not use this crate except in compliance with the License.

Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See [LICENSE](LICENSE) for
the specific language governing permissions and limitations under the License.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the [Apache-2.0
license][apache-license], shall be licensed as above, without any additional
terms or conditions.

[apache-license]: http://www.apache.org/licenses/LICENSE-2.0