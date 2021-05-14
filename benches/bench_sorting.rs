//! cargo bench --bench bench_sorting
#![feature(test)]
extern crate test;
use leetcode_rust::code_snippets::sorting::{
    bubble_sort, my_quick_sort, random_numbers_test_case, selection_sort,
};
use leetcode_rust::data_structure::heap::my_max_heap::MyMaxHeap;

#[bench]
fn bench_bubble_sort(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let mut nums = random_numbers_test_case();
        bubble_sort(&mut nums);
        //assert!(nums.is_sorted());
    });
}

#[bench]
fn bench_selection_sort(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let mut nums = random_numbers_test_case();
        selection_sort(&mut nums);
    });
}

#[bench]
fn bench_my_heap_sort(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let nums = random_numbers_test_case();
        let _ = nums.into_iter().collect::<MyMaxHeap<_>>().into_sorted_vec();
    });
}

#[bench]
fn bench_my_quick_sort(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let mut nums = random_numbers_test_case();
        my_quick_sort(0, nums.len() - 1, &mut nums);
    });
}

#[bench]
fn bench_std_heap_sort(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let nums = random_numbers_test_case();
        let _ = nums
            .into_iter()
            .collect::<std::collections::BinaryHeap<_>>()
            .into_sorted_vec();
    });
}

#[bench]
fn bench_std_merge_sort(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let mut nums = random_numbers_test_case();
        #[allow(clippy::stable_sort_primitive)]
        nums.sort();
    });
}

#[bench]
fn bench_std_quick_sort(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let mut nums = random_numbers_test_case();
        nums.sort_unstable();
        //assert!(nums.is_sorted());
    });
}
