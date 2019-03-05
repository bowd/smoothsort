use crate::numbers;

pub type Tree = (u64, usize, usize);

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
                    Some((1, 1, offset)) => (1, 0, offset+1),
                    Some((size, _, offset)) => (1, 1, offset+(*size as usize)),
                    None => (1, 1, 0),
                }
            )
        } else {
            self.collapse_last()
        }
    }

    pub fn pop(&mut self) {
        match self.trees.pop() {
            Some((1, _, _)) => { }
            Some((_, index, offset)) => {
                let pl1 = numbers::get(index-1);
                let pl2 = numbers::get(index-2);

                self.trees.push((pl1, index-1, offset));
                self.trees.push((pl2, index-2, offset + (pl1 as usize)));
            },
            None => {}
        }
    }

    fn collapse_last(&mut self) {
        let (last_size, _, _) = self.trees.pop().unwrap();
        let (prev_last_size, index, offset) = self.trees.pop().unwrap();
        self.trees.push((last_size + prev_last_size + 1, index + 1, offset))
    }

    fn can_collapse(&self) -> bool {
        if self.trees.len() < 2 {
            return false
        }

        let (_, ln_last_index, _) = self.trees[self.trees.len()-1];
        let (_, ln_prev_last_index, _) = self.trees[self.trees.len()-2];
        ln_prev_last_index == ln_last_index + 1
    }

    pub fn tree_at(&self, index: usize) -> Option<&Tree> {
        self.trees.get(index)
    }

    pub fn left_child(&self, tree: Tree) -> Option<Tree> {
        match tree {
            (1, _, _) => None,
            (_, ln_index, offset) => Some(
                (
                    numbers::get(ln_index-1),
                    ln_index-1,
                    offset,
                )
            ),
        }
    }

    pub fn right_child(&self, tree: Tree) -> Option<Tree> {
        match tree {
            (1, _, _) => None,
            (_, ln_index, offset) => Some(
                (
                    numbers::get(ln_index-2),
                    ln_index-2,
                    offset + numbers::get(ln_index-1) as usize,
                ),
            ),
        }
    }

    pub fn root_offset(&self, tree: Tree) -> usize {
        let (size, _, offset) = tree;
        offset + (size as usize) -1
    }
}

