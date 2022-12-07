use crate::algorithm::insertion::{insert_data, insert_node};
use crate::algorithm::query::nearest_neighbors;
use crate::algorithm::split::split;
use crate::node::Node;
use crate::params::Params;
use ordered_float::Float;
use std::fmt::Debug;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

pub struct SRTree<T> {
    root: Option<Node<T>>,
    params: Params,
}

#[allow(dead_code)]
impl<T> SRTree<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    #[must_use]
    pub fn new(
        min_number_of_elements: usize,
        max_number_of_elements: usize,
        reinsert_count: usize,
        prefer_close_reinsert: bool,
    ) -> SRTree<T> {
        SRTree {
            root: None,
            params: Params::new(
                min_number_of_elements,
                max_number_of_elements,
                reinsert_count,
                prefer_close_reinsert,
            ),
        }
    }

    pub fn insert(&mut self, point: &[T]) {
        if self.root.is_none() {
            self.root = Some(Node::new_leaf(point, self.params.max_number_of_elements));
        }
        let root = self.root.as_mut().unwrap();
        insert_data(root, point, &self.params);
        if root.immed_children() > self.params.max_number_of_elements {
            let sibling = split(root, &root.get_sphere().center.clone(), &self.params);
            let mut new_root = Node::new_node(
                &root.get_sphere().center,
                self.params.max_number_of_elements,
                root.get_height() + 1,
            );
            insert_node(&mut new_root, self.root.take().unwrap(), &self.params);
            insert_node(&mut new_root, sibling, &self.params);
            self.root = Some(new_root);
        }
    }

    pub fn query(&self, point: &[T], k: usize) -> Vec<Vec<T>> {
        let mut neighbors = Vec::with_capacity(k);
        if self.root.is_some() {
            nearest_neighbors(self.root.as_ref().unwrap(), point, k, &mut neighbors);
        }
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, Write};
    use std::{fmt::format, fs::File};

    #[test]
    pub fn test_insertion_query() {
        let mut tree: SRTree<f64> = SRTree::new(3, 10, 3, true);
        let search_point = vec![1.0, 0.0];
        assert!(!tree.query(&search_point, 1).contains(&search_point)); // not inserted yet
        tree.insert(&vec![1.0, 0.0]);
        assert!(tree.query(&search_point, 1).contains(&search_point)); // inserted
    }

    fn dfs(node: &Node<f64>, file: &mut File) {
        let line = format!(
            "{:?},Rect,{:?},{:?},{:?},{:?}",
            node.get_height(),
            node.get_rect().low[0],
            node.get_rect().low[1],
            node.get_rect().high[0],
            node.get_rect().high[1]
        );
        writeln!(file, "{line}");
        let line = format!(
            "{:?},Sphere,{:?},{:?},{:?}",
            node.get_height(),
            node.get_sphere().center[0],
            node.get_sphere().center[1],
            node.get_sphere().radius
        );
        writeln!(file, "{line}");
        if node.is_leaf() {
            node.points().iter().for_each(|point| {
                let line = format!(
                    "{:?},Point,{:?},{:?}",
                    node.get_height(),
                    point[0],
                    point[1]
                );
                writeln!(file, "{line}");
            });
        } else {
            node.nodes().iter().for_each(|child| {
                dfs(child, file);
            });
        }
    }

    #[test]
    pub fn test_with_real_world_dataset() {
        let mut points = Vec::new();
        // Cities in Korea (lat, lng):
        points.push(vec![34.7368, 127.7458]);
        points.push(vec![35.1928, 128.0847]);
        points.push(vec![37.3417, 127.9208]);
        points.push(vec![37.4772, 126.8664]);
        points.push(vec![36.3500, 126.9167]);
        points.push(vec![35.9439, 126.9544]);
        points.push(vec![35.3386, 129.0386]);
        points.push(vec![37.3675, 126.9469]);
        points.push(vec![37.8747, 127.7342]);
        points.push(vec![35.8167, 128.7333]);
        points.push(vec![35.9786, 126.7114]);
        points.push(vec![34.7607, 127.6622]);
        points.push(vec![34.9506, 127.4875]);
        points.push(vec![35.8500, 129.2167]);
        points.push(vec![34.7936, 126.3886]);
        points.push(vec![37.1450, 127.0694]);
        points.push(vec![37.7556, 128.8961]);
        points.push(vec![36.9706, 127.9322]);
        points.push(vec![37.2792, 127.4425]);
        points.push(vec![37.0078, 127.2797]);
        points.push(vec![37.5947, 127.1428]);
        points.push(vec![36.7817, 126.4522]);
        points.push(vec![37.8944, 127.1992]);
        points.push(vec![36.5656, 128.7250]);
        points.push(vec![37.3447, 126.9683]);
        points.push(vec![37.5392, 127.2147]);
        points.push(vec![33.2497, 126.5600]);
        points.push(vec![34.9403, 127.7017]);
        points.push(vec![34.8458, 128.4236]);
        points.push(vec![37.1361, 128.2119]);
        points.push(vec![36.2039, 127.0847]);
        points.push(vec![35.2347, 128.3575]);
        points.push(vec![35.5653, 126.8561]);
        points.push(vec![37.2939, 127.6383]);
        points.push(vec![36.8747, 128.5864]);
        points.push(vec![35.4933, 128.7489]);
        points.push(vec![36.4153, 128.1606]);
        points.push(vec![36.3333, 126.6167]);
        points.push(vec![36.6092, 127.2919]);
        points.push(vec![36.6009, 126.6650]);
        points.push(vec![34.9897, 126.4714]);
        points.push(vec![37.9133, 127.0633]);
        points.push(vec![35.0283, 126.7175]);
        points.push(vec![38.2083, 128.5911]);
        points.push(vec![36.5939, 128.2014]);
        points.push(vec![37.4406, 129.1708]);
        points.push(vec![37.4289, 126.9892]);
        points.push(vec![37.1731, 128.9861]);
        points.push(vec![37.0013, 129.3449]);
        points.push(vec![36.9353, 127.6897]);

        println!("Cities count: {:?}", points.len());
        let mut tree = SRTree::new(5, 12, 5, true);
        for p in points.iter_mut() {
            tree.insert(p);
        }

        let path = "cities.txt";
        let mut file = File::create(path).unwrap();
        dfs(&tree.root.unwrap(), &mut file);
    }
}