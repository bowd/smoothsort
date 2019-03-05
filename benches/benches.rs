#![feature(test)]

extern crate test;
use test::Bencher;
use rand::Rng;

const ITERATIONS: usize = 5;

#[bench]
fn benchmark_smooth_sort_10000(b: &mut Bencher) {
    for _ in 0..ITERATIONS {
        b.iter(move || {
            let source = gen_source(10000);
            smoothsort::sort(source)
        })
    }
}

#[bench]
fn benchmark_smooth_sort_best_case_10000(b: &mut Bencher) {
    for _ in 0..ITERATIONS {
        b.iter(|| {
            let source = gen_sorted_source(10000);
            smoothsort::sort(source)
        })
    }
}

#[bench]
fn benchmark_std_sort_10000(b: &mut Bencher) {
    for _ in 0..ITERATIONS {
        b.iter(|| {
            let mut source = gen_source(10000);
            source.sort();
        })
    }
}

#[bench]
fn benchmark_std_sort_best_case_10000(b: &mut Bencher) {
    for _ in 0..ITERATIONS {
        b.iter(|| {
            let mut source = gen_sorted_source(10000);
            source.sort();
        })
    }
}

fn gen_source(size: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut source = gen_sorted_source(size);
    rng.shuffle(source.as_mut_slice());
    source
}

fn gen_sorted_source(size: u32) -> Vec<u32> {
    let source: Vec<u32> = (0..size).collect();
    source
}
