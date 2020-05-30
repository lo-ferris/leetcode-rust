#![feature(test)]
extern crate test;
// cargo +nightly bench --bench two_sum_hashmap_vs_treemap

extern crate leetcode;
use leetcode::collections::tree_map_two_sum::*;

#[bench]
fn bench_test_two_sum_btree_map(bencher: &mut test::Bencher) {
    bencher.iter(|| two_sum_btree_map(vec![-3, 4, 3, 90], 0));
}

/*
test bench_two_sum_btreemap ... bench:         345 ns/iter (+/- 33)
test bench_two_sum_hashmap  ... bench:         540 ns/iter (+/- 53)
*/
#[bench]
fn bench_two_sum_hashmap(bencher: &mut test::Bencher) {
    bencher.iter(|| two_sum_hashmap(vec![-3, 4, 3, 90], 0));
}
