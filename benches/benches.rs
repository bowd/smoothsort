#![feature(test)]

extern crate test;
use test::Bencher;
use rand::Rng;

#[bench]
fn benchmark_lh_sort_10000(b: &mut Bencher) {
    for _ in 0..10 {
        b.iter(move || {
            let source = gen_source(10000);
            leonardo_heaps::sort(source)
        })
    }
}

#[bench]
fn benchmark_std_sort_10000(b: &mut Bencher) {
    for _ in 0..10 {
        b.iter(|| {
            let mut source = gen_source(10000);
            source.sort();
        })
    }
}

#[bench]
fn benchmark_lh_sort_best_case_10000(b: &mut Bencher) {
    for _ in 0..10 {
        b.iter(|| {
            let mut source = gen_source(10000);
            source.sort();
            leonardo_heaps::sort(source)
        })
    }
}


fn gen_source(size: usize) -> Vec<u32> {
    let mut source: Vec<u32> = Vec::with_capacity(size);
    for _ in 0..size {
        source.push(rand::thread_rng().gen_range(0, 10000));
    }
    source
}
