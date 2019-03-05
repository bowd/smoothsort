#[test]
fn heap_shape_grows_and_shrinks_correctly() {
    let mut shape = super::HeapShape::new();
    shape.push();
    assert_eq!(shape.trees, vec!((1, 1, 0)));
    shape.push();
    assert_eq!(shape.trees, vec!((1, 1, 0), (1, 0, 1)));
    shape.push();
    assert_eq!(shape.trees, vec!((3, 2, 0)));
    shape.push();
    assert_eq!(shape.trees, vec!((3, 2, 0), (1, 1, 3)));
    shape.push();
    assert_eq!(shape.trees, vec!((5, 3, 0)));
    shape.push();
    assert_eq!(shape.trees, vec!((5, 3, 0), (1, 1, 5)));
    shape.push();
    assert_eq!(shape.trees, vec!((5, 3, 0), (1, 1, 5), (1, 0, 6)));
    shape.push();
    assert_eq!(shape.trees, vec!((5, 3, 0), (3, 2, 5)));
    shape.push();
    assert_eq!(shape.trees, vec!((9, 4, 0)));
    shape.pop();
    assert_eq!(shape.trees, vec!((5, 3, 0), (3, 2, 5)));
    shape.pop();
    assert_eq!(shape.trees, vec!((5, 3, 0), (1, 1, 5), (1, 0, 6)));
    shape.pop();
    assert_eq!(shape.trees, vec!((5, 3, 0), (1, 1, 5)));
    shape.pop();
    assert_eq!(shape.trees, vec!((5, 3, 0)));
    shape.pop();
    assert_eq!(shape.trees, vec!((3, 2, 0), (1, 1, 3)));
    shape.pop();
    assert_eq!(shape.trees, vec!((3, 2, 0)));
}

fn tree_shape_of_size(size: usize) -> super::HeapShape {
    let mut shape = super::HeapShape::new();
    for _ in 0..(size+1) {
        shape.push();
    }
    shape
}

#[test]
fn tree_at_works() {
    let shape = tree_shape_of_size(11);
    let tree = shape.tree_at(0).unwrap();
    assert_eq!(*tree, (9, 4, 0));
    let tree = shape.tree_at(1).unwrap();
    assert_eq!(*tree, (3, 2, 9));
}

#[test]
fn root_offset_works() {
    let shape = tree_shape_of_size(11);
    let tree = shape.tree_at(0).unwrap();
    assert_eq!(shape.root_offset(*tree), 8);
    let tree = shape.tree_at(1).unwrap();
    assert_eq!(shape.root_offset(*tree), 11);
}


