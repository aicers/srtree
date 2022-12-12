use srtree::{SRTree, Params};

#[test]
#[should_panic]
pub fn test_invalid_dimensions() {
    let params = Params::new(3, 7, 3,true).unwrap();
    let mut tree = SRTree::new(params);
    tree.insert(&[0., 1.]);  // 2D insertion 
    tree.insert(&[0., 1., 2.]); // now 3D insertion should fail
}
