# rustica
Chessica in Rust

## Performance Notes
To allow non-root users to temporarily run `perf`, run this command:
```
sudo sh -c 'echo -1 >/proc/sys/kernel/perf_event_paranoid'
```

Then run:
```
perf record --call-graph=dwarf ./target/release/chessica
perf report --hierarchy -M intel
```

## View Assembly

I am using `cargo asm` to view generated assembly.

## Latest Profiling Data
```
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `3195903271`,
 right: `3195901860`', src/reset/profiling.rs:12:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

real	2m36.864s
user	2m36.760s
sys	0m0.016s
```

## Profiling using only `is_safe`
```
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `3195902959`,
 right: `3195901860`', src/reset/profiling.rs:12:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

real	2m53.113s
user	2m53.108s
sys	0m0.000s
```

## Processor

8th Generation Intel Core i7-8 700 6-Core Processor (12MB Cache, up to 4.6 GHz)

## Integration Tests

To run the full integration test suite, run:
```
cargo test -- --include-ignored
```

## Notes for performance testing Reset Initialization
cargo asm 'chessica::reset::child::<impl chessica::reset::Reset>::init_child'
cargo asm 'chessica::reset::child::<impl chessica::reset::Reset>::init_child' | wc -l
cargo asm chessica::reset::new
cargo asm chessica::reset::new | wc -l

