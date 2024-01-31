use criterion::{criterion_group, criterion_main, Criterion};

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum_complex1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m: HashMap<i32, i32> = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            match m.get(&(target - *v)) {
                Some(&i2) => return vec![i as i32, i2],
                None => m.insert(*v, i as i32),
            };
        }
        vec![]
    }

    pub fn two_sum_complex2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // [1] prepare numbers for binary search by
        //     saving original indices and sorting
        let mut nums: Vec<(usize, i32)> =
            nums.into_iter().enumerate().collect::<Vec<(usize, i32)>>();

        nums.sort_unstable_by_key(|&(_, n)| n);

        // [2] we perform linear scan for the first number
        //     and do binary search for the second number
        for (k, (i, ni)) in nums.iter().enumerate() {
            // [3] to prevent duplication, slices start from k+1
            match nums[k + 1..].binary_search_by_key(&(target - *ni), |&(_, nj)| nj) {
                Ok(jj) => return vec![*i as i32, nums[jj + k + 1].0 as i32],
                Err(_) => {}
            }
        }

        return vec![0, 0];
    }

    // simple solution
    pub fn two_sum_simple(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, i_val) in nums.iter().enumerate() {
            for j in i + 1..nums.len() {
                if i_val + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}

fn benchmark_two_sum_complex(c: &mut Criterion) {
    // Set up any necessary data for your benchmark
    let nums = vec![2, 9, 11, 15];
    let target = 11;

    // Define the benchmarked function
    c.bench_function("two_sum complex #1", |b| {
        b.iter(|| Solution::two_sum_complex1(nums.clone(), target))
    });
}
fn benchmark_two_sum_complex2(c: &mut Criterion) {
    // Set up any necessary data for your benchmark
    let nums = vec![2, 9, 11, 15];
    let target = 11;

    // Define the benchmarked function
    c.bench_function("two_sum complex #2", |b| {
        b.iter(|| Solution::two_sum_complex2(nums.clone(), target))
    });
}
fn benchmark_two_sum(c: &mut Criterion) {
    // Set up any necessary data for your benchmark
    let nums = vec![2, 9, 11, 15];
    let target = 11;

    // Define the benchmarked function
    c.bench_function("two_sum", |b| {
        b.iter(|| Solution::two_sum_simple(nums.clone(), target))
    });
}

// Define the criterion group
criterion_group!(bench_simple, benchmark_two_sum);
criterion_group!(bench_complex, benchmark_two_sum_complex);
criterion_group!(bench_complex2, benchmark_two_sum_complex2);

// Run the benchmarks
criterion_main!(bench_simple, bench_complex, bench_complex2);
