#![feature(stdarch)]
use std::arch::x86_64::*;

const LIMIT: usize = 20_00_00;

fn rdtsc() -> u64 {
    unsafe { _rdtsc() }
}

pub fn main() {
    let mut arr1: [f64; LIMIT] = [0f64; LIMIT];
    let mut arr2: [f64; LIMIT] = [0f64; LIMIT];

    let it = rdtsc();

    for i in 0..LIMIT / 4 {
        unsafe {
            let array1 = _mm256_loadu_pd(arr1.get_unchecked_mut(i * 4));
            let array2 = _mm256_loadu_pd(arr2.get_unchecked_mut(i * 4));

            let add = _mm256_set_pd(
                (i * 4 + 1) as f64,
                (i * 4 + 2) as f64,
                (i * 4 + 3) as f64,
                (i * 4 + 4) as f64,
            );

            _mm256_storeu_pd(arr1.get_unchecked_mut(i * 4), _mm256_add_pd(array1, add));
            _mm256_storeu_pd(arr2.get_unchecked_mut(i * 4), _mm256_add_pd(array2, add));
        }
    }

    let mut dot_p: f64 = 0.0;
    let now = std::time::Instant::now();

    // let cycles = rdtsc();
    for i in 0..LIMIT / 4 {
        unsafe {
            let array1 = _mm256_loadu_pd(arr1.get_unchecked_mut(i * 4));
            let array2 = _mm256_loadu_pd(arr2.get_unchecked_mut(i * 4));
            let result = _mm256_mul_pd(array1, array2);
            let shuffle = _mm256_shuffle_pd(result, result, _MM_SHUFFLE(2, 3, 0, 1));
            let sums = _mm256_add_pd(result, shuffle);
            let shuffle = _mm256_shuffle_pd(sums, sums, _MM_SHUFFLE(0, 1, 3, 2));
            let sums = _mm256_add_pd(sums, shuffle);
            let result = _mm256_cvtsd_f64(sums);
            dot_p += result;
            // let result = std::mem::transmute::<__m256d, [f64; 4]>(result);
            // dot_p += *result.get_unchecked(0) as f64
            //     + *result.get_unchecked(1) as f64
            //     + *result.get_unchecked(2) as f64
            //     + *result.get_unchecked(3) as f64;
        };
    }

    // println!("{}", rdtsc() - cycles);
    println!("{}", now.elapsed().as_micros());
    println!("{}", dot_p);
}
