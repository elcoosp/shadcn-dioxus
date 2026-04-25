#[test]
fn test_skeleton_placeholder() { assert!(true); }

use crate::SkeletonProps;

#[test]
fn test_skeleton_props_exist() {
    let _ = SkeletonProps {
        class: "h-4 w-4".into(),
        attributes: Vec::new(),
    };
    assert!(true);
}
