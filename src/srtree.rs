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
    use std::io::{Error, Write, BufReader, BufRead};
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
    pub fn test_with_world_dataset() {
        let mut tree = SRTree::new(5, 12, 5, true);

        let file = File::open("worldcities.csv").unwrap();
        let reader = BufReader::new(file);

        let mut skip = true;
        for line in reader.lines() {
            if skip {
                skip = false;
                continue;
            }
            if line.is_ok() {
                let mut p = Vec::new();
                let mut line = line.as_ref().unwrap();
                let mut population: usize = 0;
                let mut country = "";

                for (i, val) in line.split(",").enumerate(){
                    let mut chars = val.chars();
                    chars.next();
                    chars.next_back();
                    let val = chars.as_str();
                    if i == 2 || i == 3 {
                        let c: f64 = val.parse().unwrap_or(f64::infinity());
                        if c != f64::infinity() {
                            p.push(c);
                        }
                    }

                    if i == 4 {
                        country = val;
                    }

                    if i == 9 {
                        population = val.parse().unwrap_or(0);
                    }
                }

                if p.len() == 2 && population >= 50_000 && country == "United States" {
                    tree.insert(&p);
                }
            }
        }

        let path = "cities.txt";
        let mut file = File::create(path).unwrap();
        dfs(&tree.root.unwrap(), &mut file);
    }
}