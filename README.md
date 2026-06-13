# weird_bench

This repository serves as documentation for unexpected behavior of `fat` [LTO](https://doc.rust-lang.org/cargo/reference/profiles.html#lto) option during benchmarking.


Release: https://doc.rust-lang.org/cargo/reference/profiles.html#release
```
cargo bench --test bench --profile release

running 2 tests
test gcd_naive_2_test ... bench:          47.88 ns/iter (+/- 2.86)
test gcd_naive_test   ... bench:          56.33 ns/iter (+/- 1.72)
```
Bench: ./Cargo.toml

```
cargo bench --test bench --profile bench

test gcd_naive_2_test ... bench:           0.25 ns/iter (+/- 0.01)
test gcd_naive_test   ... bench:           0.25 ns/iter (+/- 0.01)
```

None of methods appears to be optimizable to this point by optimizing linked libraries.

## Code

- Methods: [./src/lib.rs](./src/lib.rs)
- Bench: [./tests/bench.rs](./tests/bench.rs)
- Cargo TOML: [./Cargo.toml](./Cargo.toml)

