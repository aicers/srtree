# srtree
Rust implementation of SR-Tree: nearest neighbor search index for high-dimensional clustered datasets.

## Example
This example shows how to query nearest neighbors:
```rust
use srtree::{Params, SRTree};

fn main() {
    let params = Params::new(7, 15, 7, true).unwrap();
    let mut tree: SRTree<f64> = SRTree::with_params(params);
    tree.insert(&[0., 0.], 0);
    tree.insert(&[1., 1.], 1);
    tree.insert(&[2., 2.], 2);
    tree.insert(&[3., 3.], 3);
    tree.insert(&[4., 4.], 4);

    let (indices, distances) = tree.query(&[8., 8.], 3);
    println!("{indices:?}"); // [4, 3, 2] (sorted by distance)
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