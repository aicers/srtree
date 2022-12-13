use srtree::{InsertionResult, Params, SRTree};

#[test]
pub fn test_insertion_invalid_dimensions() {
    let params = Params::new(3, 7, 3, true).unwrap();
    let mut tree = SRTree::new(2, params);

    let result = tree.insert(&[0., 1.]); // valid insertion
    assert!(matches!(result, InsertionResult::Success));

    let result = tree.insert(&[0., 1., 2.]); // now 3D insertion should fail
    assert!(matches!(result, InsertionResult::Failure));
}
