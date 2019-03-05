use leonardo_heaps::LeonardoHeap;
use rand::Rng;

#[test]
fn sort() {
    let source = gen_source(100);
    let source = leonardo_heaps::sort(source);

    let mut expected = source.clone();
    expected.sort();
    assert_eq!(source, expected);
}

#[test]
fn sort_by() {
    let source = gen_source(100);
    let source = leonardo_heaps::sort_by(source, |a, b| b > a);

    let mut expected = source.clone();
    expected.sort_by(|a, b| b.cmp(a));
    assert_eq!(source, expected);
}

#[test]
fn heap_from_vec() {
    let source = vec![6,7,8,9];
    let _ = LeonardoHeap::from_vec(source);
}

#[test]
fn heap_new() {
    let mut heap = LeonardoHeap::new();
    heap.push('a');
    heap.push('b');
    assert_eq!(heap.pop(), Some('b'));
    assert_eq!(heap.pop(), Some('a'));
    assert_eq!(heap.pop(), None);
}

fn gen_source(size: usize) -> Vec<u32> {
    let mut source: Vec<u32> = Vec::with_capacity(size);
    for _ in 0..size {
        source.push(rand::thread_rng().gen_range(0, 10000));
    }
    source
}


#[test]
fn reversed() {
    let mut source: Vec<u32> = Vec::new();
    for _ in 0..100 {
        source.push(rand::thread_rng().gen_range(0, 10000));
    }
    let mut heap = LeonardoHeap::from_vec_by(source, |a, b| a < b);
    let mut last = heap.pop().unwrap();

    while let Some(current) = heap.pop() {
        assert!(last <= current, "{} <= {}", last, current);
        last = current;
    }

}
