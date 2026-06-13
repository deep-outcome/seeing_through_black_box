#![feature(test)]

use weird_bench::{gcd_naive, gcd_naive_2};

extern crate test;
use test::Bencher;

#[bench]
fn gcd_naive_test(b: &mut Bencher) {
    let num_1 = 2_559_031_471u64; // 150531263ᵖ ⋅17ᵖ
    let num_2 = 1_956_912_061; // 150531697ᵖ ⋅13ᵖ    

    b.iter(|| gcd_naive(num_1, num_2));
}

#[bench]
fn gcd_naive_2_test(b: &mut Bencher) {
    let num_1 = 2_559_031_471u64; // 150531263ᵖ ⋅17ᵖ
    let num_2 = 1_956_912_061; // 150531697ᵖ ⋅13ᵖ

    b.iter(|| _ = gcd_naive_2(num_1, num_2));
}
