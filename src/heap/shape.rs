use crate::numbers;

pub type Tree = (usize, usize);

pub struct HeapShape {
    pub trees: Vec<Tree>,
}

impl HeapShape {
    pub fn new() -> HeapShape {
        HeapShape{
            trees: Vec::new(),
        }
    }

    pub fn push(&mut self) {
        if !self.can_collapse() {
            self.trees.push(
                match self.trees.last() {
                    Some((1, offset)) => (0, offset+1),
                    Some((index, offset)) => (1, offset+(numbers::get(*index) as usize)),
                    None => (1, 0),
                }
            )
        } else {
            self.collapse_last()
        }
    }

    pub fn pop(&mut self) {
        match self.trees.pop() {
            Some((0, _)) => { }
            Some((1, _)) => { }
            Some((index, offset)) => {
                self.trees.push((index-1, offset));
                self.trees.push((index-2, offset + (numbers::get(index-1) as usize)));
            },
            None => {}
        }
    }

    fn collapse_last(&mut self) {
        let (_, _) = self.trees.pop().unwrap();
        let (index, offset) = self.trees.pop().unwrap();
        self.trees.push((index + 1, offset))
    }

    fn can_collapse(&self) -> bool {
        if self.trees.len() < 2 {
            return false
        }

        let (ln_last_index, _) = self.trees[self.trees.len()-1];
        let (ln_prev_last_index, _) = self.trees[self.trees.len()-2];
        ln_prev_last_index == ln_last_index + 1
    }

    pub fn tree_at(&self, index: usize) -> Option<&Tree> {
        self.trees.get(index)
    }

    pub fn left_child(&self, tree: Tree) -> Option<Tree> {
        match tree {
            (0, _) => None,
            (1, _) => None,
            (ln_index, offset) => Some(
                (
                    ln_index-1,
                    offset,
                )
            ),
        }
    }

    pub fn right_child(&self, tree: Tree) -> Option<Tree> {
        match tree {
            (0, _) => None,
            (1, _) => None,
            (ln_index, offset) => Some(
                (
                    ln_index-2,
                    offset + numbers::get(ln_index-1) as usize,
                ),
            ),
        }
    }

    pub fn root_offset(&self, tree: Tree) -> usize {
        let (index, offset) = tree;
        offset + (numbers::get(index) as usize) -1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn heap_shape_grows_and_shrinks_correctly() {
        let mut shape = super::HeapShape::new();
        shape.push();
        assert_eq!(shape.trees, vec!((1, 0)));
        shape.push();
        assert_eq!(shape.trees, vec!((1, 0), (0, 1)));
        shape.push();
        assert_eq!(shape.trees, vec!((2, 0)));
        shape.push();
        assert_eq!(shape.trees, vec!((2, 0), (1, 3)));
        shape.push();
        assert_eq!(shape.trees, vec!((3, 0)));
        shape.push();
        assert_eq!(shape.trees, vec!((3, 0), (1, 5)));
        shape.push();
        assert_eq!(shape.trees, vec!((3, 0), (1, 5), (0, 6)));
        shape.push();
        assert_eq!(shape.trees, vec!((3, 0), (2, 5)));
        shape.push();
        assert_eq!(shape.trees, vec!((4, 0)));
        shape.pop();
        assert_eq!(shape.trees, vec!((3, 0), (2, 5)));
        shape.pop();
        assert_eq!(shape.trees, vec!((3, 0), (1, 5), (0, 6)));
        shape.pop();
        assert_eq!(shape.trees, vec!((3, 0), (1, 5)));
        shape.pop();
        assert_eq!(shape.trees, vec!((3, 0)));
        shape.pop();
        assert_eq!(shape.trees, vec!((2, 0), (1, 3)));
        shape.pop();
        assert_eq!(shape.trees, vec!((2, 0)));
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
        assert_eq!(*tree, (4, 0));
        let tree = shape.tree_at(1).unwrap();
        assert_eq!(*tree, (2, 9));
    }

    #[test]
    fn root_offset_works() {
        let shape = tree_shape_of_size(11);
        let tree = shape.tree_at(0).unwrap();
        assert_eq!(shape.root_offset(*tree), 8);
        let tree = shape.tree_at(1).unwrap();
        assert_eq!(shape.root_offset(*tree), 11);
    }
}

