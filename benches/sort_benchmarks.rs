use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

// Merge Sort implementation
fn merge_sort(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }
    let mid = arr.len() / 2;
    let left = merge_sort(arr[..mid].to_vec());
    let right = merge_sort(arr[mid..].to_vec());
    merge(&left, &right, &mut arr);
    arr
}

fn merge(left: &[i32], right: &[i32], arr: &mut Vec<i32>) {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut index = 0;

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] < right[right_index] {
            arr[index] = left[left_index];
            left_index += 1;
        } else {
            arr[index] = right[right_index];
            right_index += 1;
        }
        index += 1;
    }

    while left_index < left.len() {
        arr[index] = left[left_index];
        left_index += 1;
        index += 1;
    }

    while right_index < right.len() {
        arr[index] = right[right_index];
        right_index += 1;
        index += 1;
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let data: Vec<i32> = (0..1000).rev().collect(); // Reversed array of 1000 elements

    // Benchmark Merge Sort
    c.bench_function("merge_sort", |b| {
        b.iter(|| merge_sort(black_box(data.clone())))
    });

    // Benchmark Rust's built-in sort
    c.bench_function("built_in_sort", |b| {
        b.iter(|| {
            let mut data_clone = data.clone();
            black_box(data_clone.sort());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
