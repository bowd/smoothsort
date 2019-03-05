mod numbers;
mod heap;

pub use heap::{LeonardoHeap, Cmp};


pub fn sort<T: PartialOrd + std::fmt::Display + std::fmt::Debug>(items: Vec<T>) -> Vec<T> {
    let mut heap = LeonardoHeap::from_vec(items);
    heap.in_place_sort();
    heap.release()
}

pub fn sort_by<T: PartialOrd + std::fmt::Display + std::fmt::Debug>(items: Vec<T>, cmp: Cmp<T>) -> Vec<T> {
    let mut heap = LeonardoHeap::from_vec_by(items, cmp);
    heap.in_place_sort();
    heap.release()
}


