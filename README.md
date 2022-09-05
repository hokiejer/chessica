# rustica
Chessica in Rust

## Performance Notes
To allow non-root users to temporarily run `perf`, run this command:
```
sudo sh -c 'echo -1 >/proc/sys/kernel/perf_event_paranoid'
```

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
