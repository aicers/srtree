pub struct Params {
    pub min_number_of_elements: usize,
    pub max_number_of_elements: usize,
    pub reinsert_count: usize,
}

impl Params {
    pub fn new(
        min_number_of_elements: usize,
        max_number_of_elements: usize,
        reinsert_count: usize,
    ) -> Params {
        // todo: validate params
        Params { min_number_of_elements, max_number_of_elements, reinsert_count }
    }
}
