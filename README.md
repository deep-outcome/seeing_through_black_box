# Seeing Through Black Box

Compilation optimization can sometimes bring in unexpected, not pleasant experiences. Consider [benchmarking](./tests/bench.rs) simple binary GCD implementation as seen in [lib.rs](./src/lib.rs).

For instance,

```rust
#[bench]
fn gcd_naive_test_A(b: &mut Bencher) {
    let num_1 = 2_559_031_471u64; // 150531263ᵖ ⋅17ᵖ
    let num_2 = 1_956_912_061; // 150531697ᵖ ⋅13ᵖ    

    b.iter(|| gcd_naive(num_1, num_2));
}
```

Assuming `fat` [LTO](https://doc.rust-lang.org/cargo/reference/profiles.html#lto) option active for benchark build, `cargo bench --test bench --profile bench _A` can produce output like 

```
test gcd_naive_2_test_A ... bench:           0.25 ns/iter (+/- 0.01)
test gcd_naive_test_A   ... bench:           0.25 ns/iter (+/- 0.00)
```

Which is absurd and obviously wrong.

This is due compiler intelligence which affirmed it in optimizing out important test code. [`std::hint::black_box`](https://doc.rust-lang.org/std/hint/fn.black_box.html) instructs compiler to _'see such code in black box'_ thus it will not optimize it out.


```rust
#[bench]
fn gcd_naive_test_B(b: &mut Bencher) {
    let num_1 = 2_559_031_471u64; // 150531263ᵖ ⋅17ᵖ
    let num_2 = 1_956_912_061; // 150531697ᵖ ⋅13ᵖ    

    b.iter(|| gcd_naive(black_box(num_1), black_box(num_2)));
}
```

```
test gcd_naive_2_test_B ... bench:          33.34 ns/iter (+/- 4.99)
test gcd_naive_test_B   ... bench:          42.01 ns/iter (+/- 1.17)
```

Problem gone, for good.

