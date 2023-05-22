use srtree::{InsertionResult, Params, SRTree};

#[test]
pub fn test_insertion_invalid_dimensions() {
    let params = Params::new(3, 7, 3).unwrap();
    let mut tree = SRTree::with_params(params);

    let result = tree.insert(&vec![0., 1.], 0); // valid insertion
    assert!(matches!(result, InsertionResult::Success));

    let result = tree.insert(&vec![0., 1., 2.], 1); // now 3D insertion should fail
    assert!(matches!(result, InsertionResult::Failure));
}
