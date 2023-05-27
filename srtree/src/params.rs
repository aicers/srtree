#[derive(Clone, Copy)]
pub struct Params {
    pub min_number_of_elements: usize,
    pub max_number_of_elements: usize,
    pub dimension: usize,
}

impl Params {
    #[must_use]
    pub fn new(min_number_of_elements: usize, max_number_of_elements: usize) -> Option<Params> {
        if min_number_of_elements > (max_number_of_elements + 1) / 2 {
            return None;
        }
        Some(Params {
            min_number_of_elements,
            max_number_of_elements,
            dimension: 0,
        })
    }

    #[must_use]
    pub fn default_params() -> Params {
        Params {
            min_number_of_elements: 8,
            max_number_of_elements: 20,
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
        let params = Params::new(min_num_of_elements_per_node, max_num_of_elements_per_node);
        assert!(params.is_none())
    }

    #[test]
    pub fn test_with_valid_params() {
        let min_num_of_elements_per_node = 4;
        let max_num_of_elements_per_node = 10;
        let params = Params::new(min_num_of_elements_per_node, max_num_of_elements_per_node);
        assert!(params.is_some())
    }
}
