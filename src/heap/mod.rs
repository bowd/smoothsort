mod shape;
use shape::{HeapShape, Tree};

pub type Cmp<T> = fn(&T, &T) -> bool;

pub struct LeonardoHeap<T> {
    items: Vec<T>,
    shape: HeapShape,
    length: usize,
    cmp: Cmp<T>,
}

impl<T: PartialOrd + std::fmt::Display + std::fmt::Debug> LeonardoHeap<T> {
    pub fn from_vec(source: Vec<T>) -> LeonardoHeap<T> {
        let mut heap = LeonardoHeap {
            items: source,
            shape: HeapShape::new(),
            length: 0,
            cmp: |a, b| a > b,
        };

        for _ in 0..heap.items.len() {
            heap.length += 1;
            heap.balance_on_push();
        }

        heap
    }

    pub fn from_vec_by(source: Vec<T>, cmp: Cmp<T>) -> LeonardoHeap<T> {
        let mut heap = LeonardoHeap {
            items: source,
            shape: HeapShape::new(),
            length: 0,
            cmp: cmp,
        };

        for _ in 0..heap.items.len() {
            heap.length += 1;
            heap.balance_on_push();
        }

        heap
    }

    pub fn new() -> LeonardoHeap<T> {
        LeonardoHeap {
            items: Vec::new(),
            shape: HeapShape::new(),
            length: 0,
            cmp: |a, b| a > b,
        }
    }

    pub fn new_by(cmp: Cmp<T>) -> LeonardoHeap<T> {
        LeonardoHeap {
            items: Vec::new(),
            shape: HeapShape::new(),
            length: 0,
            cmp
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.items.pop() {
            Some(last) => {
                self.length -= 1;
                let len_before = self.shape.trees.len();
                self.shape.pop();
                if len_before < self.shape.trees.len() {
                    // New trees need to stringify
                    self.rectify(self.shape.trees.len() - 2);
                    self.rectify(self.shape.trees.len() - 1);
                }

                Some(last)
            }
            None => None
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
        self.length += 1;
        self.balance_on_push()
    }

    pub fn in_place_sort(&mut self) {
        while self.length > 0 {
            self.length -= 1;
            let len_before = self.shape.trees.len();
            self.shape.pop();
            if len_before < self.shape.trees.len() {
                // New trees need to stringify
                self.rectify(self.shape.trees.len() - 2);
                self.rectify(self.shape.trees.len() - 1);
            }
        }
    }

    pub fn release(self) -> Vec<T> {
        self.items
    }

    fn balance_on_push(&mut self) {
        self.shape.push();
        // We shouldn't always do a full rectify here or should I?
        self.rectify(self.shape.trees.len() - 1);
    }

    fn rectify(&mut self, offset: usize) {
        match self.shape.trees.len() {
            0 => {}
            1 => {
                self.balance(*self.shape.tree_at(0).unwrap());
            }
            _ => {
                let mut current = offset;
                for index in (0..offset).rev() {
                    if self.should_swap_heaps(current, index) {
                        self.swap_heaps(current, index);
                        current = index;
                    }
                }
                self.balance(*self.shape.tree_at(current).unwrap());
            }
        }
    }

    fn balance(&mut self, tree: Tree) {
        let left_child = self.shape.left_child(tree);
        let right_child = self.shape.right_child(tree);
        if let Some(_) = left_child {
            let left_child = left_child.unwrap();
            let right_child = right_child.unwrap();
            let left_child_offset = self.shape.root_offset(left_child);
            let right_child_offset = self.shape.root_offset(right_child);
            let tree_offset = self.shape.root_offset(tree);
            let cmp = self.cmp;

            if cmp(&self.items[left_child_offset], &self.items[tree_offset]) &&
                cmp(&self.items[left_child_offset], &self.items[right_child_offset]) {
                self.items.swap(left_child_offset, tree_offset);
                self.balance(left_child);
            } else if cmp(&self.items[right_child_offset], &self.items[tree_offset]) {
                self.items.swap(right_child_offset, tree_offset);
                self.balance(right_child);
            }
        }
    }

    fn should_swap_heaps(&self, base_tree: usize, other_tree: usize) -> bool {
        let base_tree = *self.shape.tree_at(base_tree).unwrap();
        let (has_left_child, base_tree_left_child) = match self.shape.left_child(base_tree) {
            None => (false, 0),
            Some(child) => (true, self.shape.root_offset(child)),
        };
        let (has_right_child, base_tree_right_child) = match self.shape.right_child(base_tree) {
            None => (false, 0),
            Some(child) => (true, self.shape.root_offset(child)),
        };

        let base_tree = self.shape.root_offset(base_tree);
        let other_tree = self.shape.root_offset(*self.shape.tree_at(other_tree).unwrap());
        let cmp = self.cmp;

        (
            cmp(&self.items[other_tree], &self.items[base_tree]) &&
            (!has_left_child || cmp(&self.items[other_tree], &self.items[base_tree_left_child])) &&
            (!has_right_child || cmp(&self.items[other_tree], &self.items[base_tree_right_child]))
        )
    }

    fn swap_heaps(&mut self, base_tree: usize, other_tree: usize) {
        let base_tree = self.shape.root_offset(*self.shape.tree_at(base_tree).unwrap());
        let other_tree = self.shape.root_offset(*self.shape.tree_at(other_tree).unwrap());
        self.items.swap(base_tree, other_tree);
    }
}

