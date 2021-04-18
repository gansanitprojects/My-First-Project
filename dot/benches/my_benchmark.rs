#![feature(core_intrinsics)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::arch::x86_64::*;
use std::mem::transmute;

const LIMIT: usize = 20_00_00;

fn manual(arr1: &[f64], arr2: &[f64]) -> f64 {
    let mut dotp = unsafe { _mm256_setzero_pd() };

    for i in 0..LIMIT / 4 {
        let item1 = unsafe { _mm256_loadu_pd(arr1.get_unchecked(i * 4)) };
        let item2 = unsafe { _mm256_loadu_pd(arr2.get_unchecked(i * 4)) };
        let multiplication = unsafe { _mm256_mul_pd(item1, item2) };
        dotp = unsafe { _mm256_add_pd(dotp, multiplication) };
    }

    let result = unsafe { transmute::<__m256d, [f64; 4]>(dotp) };

    let result = unsafe {
        result.get_unchecked(0)
            + result.get_unchecked(1)
            + result.get_unchecked(2)
            + result.get_unchecked(3)
    };

    result

}

fn auto(arr1: &[f64], arr2: &[f64]) -> f64 {
    let mut dotp: f64 = 0.0;

    for i in 0..LIMIT {
        dotp += arr1[i] * arr2[i];
    }
    
    dotp
}

fn intrinsics(arr1: &[f64], arr2: &[f64]) -> f64 {
    assert_eq!(arr1.len(), arr2.len());
    let mut dot_p = 0.0;
    for (&a, &b) in arr1.iter().zip(arr2.iter()) {
        use std::intrinsics::{fadd_fast, fmul_fast};
        unsafe {
            dot_p = fadd_fast(dot_p, fmul_fast(a, b));
        }
    }
    dot_p
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut arr1: [f64; LIMIT] = [0f64; LIMIT];
    let mut arr2: [f64; LIMIT] = [0f64; LIMIT];

    for i in 0..LIMIT {
        arr1[i] = (i + 1) as f64;
        arr2[i] = (i + 1) as f64;
    }

    c.bench_function("auto", |b| b.iter(|| auto(&arr1, &arr2)));
    c.bench_function("manual", |b| b.iter(|| manual(&arr1, &arr2)));
    c.bench_function("intrinsics", |b| b.iter(|| intrinsics(&arr1, &arr2)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
