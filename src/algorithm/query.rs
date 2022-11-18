use crate::algorithm::distance::euclidean;
use crate::node::Node;
use ordered_float::{Float, OrderedFloat};
use priority_queue::DoublePriorityQueue;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

pub fn nearest_neighbors<T>(node: &Node<T>, point: &Vec<T>, k: usize, result: &mut Vec<Vec<T>>)
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if node.is_leaf() {
        // construct a queue with distance as a priority
        let mut queue: DoublePriorityQueue<usize, OrderedFloat<T>> = DoublePriorityQueue::new();
        for (index, candidate) in node.points().iter().enumerate() {
            queue.push(index, OrderedFloat(euclidean(candidate, point)));
        }

        // keep selecting the next nearest item until k neighbors are found:
        while !queue.is_empty() {
            let (index, _) = queue.pop_min().unwrap();
            if result.len() < k {
                result.push(node.points()[index].clone());
            } else {
                break;
            }
        }
    }else{
        todo!("query from a node");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::Node;
    use crate::algorithm::insertion::insert;

    #[test]
    pub fn test_nearest_neighbors_with_leaf(){
        let origin = vec![0., 0.];
        let maximum_num_of_elements = 100;
        let mut leaf_node = Node::new_leaf(&origin, maximum_num_of_elements);

        for i in 0..maximum_num_of_elements {
            let point = vec![i as f64, 0.];
            insert(&mut leaf_node, &point);
        }

        let mut result = Vec::new();
        let k = maximum_num_of_elements / 3;
        nearest_neighbors(&leaf_node, &origin, k, &mut result);

        assert!(result.len() == k);
        for i in 0..k {
            let point = vec![i as f64, 0.];
            assert!(result.contains(&point));
        }
    }
}