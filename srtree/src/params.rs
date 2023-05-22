#[derive(Clone, Copy)]
pub struct Params {
    pub min_number_of_elements: usize,
    pub max_number_of_elements: usize,
    pub reinsert_count: usize,
    pub dimension: usize,
}

impl Params {
    #[must_use]
    pub fn new(
        min_number_of_elements: usize,
        max_number_of_elements: usize,
        reinsert_count: usize,
    ) -> Option<Params> {
        if min_number_of_elements > (max_number_of_elements + 1) / 2
            || reinsert_count >= max_number_of_elements - min_number_of_elements
        {
            return None;
        }
        Some(Params {
            min_number_of_elements,
            max_number_of_elements,
            reinsert_count,
            dimension: 0,
        })
    }

    #[must_use]
    pub fn default_params() -> Params {
        Params {
            min_number_of_elements: 8,
            max_number_of_elements: 20,
            reinsert_count: 6,
            dimension: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_with_invalid_min_num_elements() {
        let min_num_of_elements_per_node = 6;
        let max_num_of_elements_per_node = 10;
        let reinsert_count = 4;
        let params = Params::new(
            min_num_of_elements_per_node,
            max_num_of_elements_per_node,
            reinsert_count,
        );
        assert!(params.is_none())
    }

    #[test]
    pub fn test_with_invalid_reinsert_count() {
        let min_num_of_elements_per_node = 4;
        let max_num_of_elements_per_node = 10;
        let reinsert_count = 7;
        let params = Params::new(
            min_num_of_elements_per_node,
            max_num_of_elements_per_node,
            reinsert_count,
        );
        assert!(params.is_none())
    }

    #[test]
    pub fn test_with_valid_params() {
        let min_num_of_elements_per_node = 4;
        let max_num_of_elements_per_node = 10;
        let reinsert_count = 5;
        let params = Params::new(
            min_num_of_elements_per_node,
            max_num_of_elements_per_node,
            reinsert_count,
        );
        assert!(params.is_some())
    }
}
