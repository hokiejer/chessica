# rustica

Chessica in Rust

## Performance Notes

To allow non-root users to temporarily run `perf`, run this command:

```bash
sudo sh -c 'echo -1 >/proc/sys/kernel/perf_event_paranoid'
```

Then run:

```bash
cargo build --profile burn
perf record --call-graph=dwarf ./target/burn/chessica
perf report --hierarchy -M intel
```

## View Assembly

I am using `cargo asm` to view generated assembly.

## Latest Profiling Data

```text
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `3195903271`,
 right: `3195901860`', src/reset/profiling.rs:12:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

real  2m36.864s
user  2m36.760s
sys   0m0.016s
```

## Profiling using only `is_safe`

```text
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `3195902959`,
 right: `3195901860`', src/reset/profiling.rs:12:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

real  2m53.113s
user  2m53.108s
sys   0m0.000s
```

## Processor

My processor = 8th Generation Intel Core i7-8 700 6-Core Processor (12MB Cache, up to 4.6 GHz)

To take advantage of native CPU optimizations, add the following to `~/.profile`:

```bash
# Use native CPU optimizations in Rust compilation
export RUSTFLAGS='-C target-cpu native'
```

## Integration Tests

To run the full integration test suite, run:

```bash
cargo test -- --include-ignored
```

## Notes for performance testing Reset Initialization

```bash
cargo asm 'chessica::reset::child::<impl chessica::reset::Reset>::init_child'
cargo asm 'chessica::reset::child::<impl chessica::reset::Reset>::init_child' | wc -l
cargo asm chessica::reset::new
cargo asm chessica::reset::new | wc -l
```

## Score Data

(From In Place Alpha Beta)
1      975          (20)
2     -586         (114)
3      880        (1862)
4     -794       (25469)
5   999021      (208981)
6  -999033     (3269415)
7   999410    (14587601)
8  -999134   (311010746)
9   999592  (1069958508)
10 -999259 (25679699729)

Running profile script for Iterative Keep Depth Alpha-Beta...
Search Depth == 9, Keep Depth == 4
i == 1
Score == 975
Move count == 20
i == 2
Score == -586
Move count == 154
i == 3
Score == 880
Move count == 2081
i == 4
Score == -794
Move count == 28216
i == 5
Score == 999021
Move count == 122433
i == 6
Score == -999033
Move count == 3422085
i == 7
Score == 999410
Move count == 7522085
i == 8
Score == -999134
Move count == 324410163
i == 9
Score == 999592
Move count == 985531719
Score == 999592

hokiejer ➜ /workspaces/rustica (iterative_ab_fix) $ target/release/chessica --profile keep-depth-ab --ab-search-depth 1
Running profile script for Keep Depth Alpha-Beta...
Search Depth == 1, Keep Depth == 4
Score == 975  Move count == 20
@hokiejer ➜ /workspaces/rustica (iterative_ab_fix) $ target/release/chessica --profile keep-depth-ab --ab-search-depth 2
Running profile script for Keep Depth Alpha-Beta...
Search Depth == 2, Keep Depth == 4
Score == -586  Move count == 114
@hokiejer ➜ /workspaces/rustica (iterative_ab_fix) $ target/release/chessica --profile keep-depth-ab --ab-search-depth 3
Running profile script for Keep Depth Alpha-Beta...
Search Depth == 3, Keep Depth == 4
Score == 880  Move count == 1862
@hokiejer ➜ /workspaces/rustica (iterative_ab_fix) $ target/release/chessica --profile keep-depth-ab --ab-search-depth 4
Running profile script for Keep Depth Alpha-Beta...
Search Depth == 4, Keep Depth == 4
Score == -794  Move count == 25469
@hokiejer ➜ /workspaces/rustica (iterative_ab_fix) $ target/release/chessica --profile keep-depth-ab --ab-search-depth 5
Running profile script for Keep Depth Alpha-Beta...
Search Depth == 5, Keep Depth == 4
Score == 999021  Move count == 208296
@hokiejer ➜ /workspaces/rustica (iterative_ab_fix) $ target/release/chessica --profile keep-depth-ab --ab-search-depth 6
Running profile script for Keep Depth Alpha-Beta...
Search Depth == 6, Keep Depth == 4
Score == -999033  Move count == 3259958
@hokiejer ➜ /workspaces/rustica (iterative_ab_fix) $ target/release/chessica --profile keep-depth-ab --ab-search-depth 7
Running profile script for Keep Depth Alpha-Beta...
Search Depth == 7, Keep Depth == 4
Score == 999410  Move count == 14527329
@hokiejer ➜ /workspaces/rustica (iterative_ab_fix) $ target/release/chessica --profile keep-depth-ab --ab-search-depth 8
Running profile script for Keep Depth Alpha-Beta...
Search Depth == 8, Keep Depth == 4
Score == -999134  Move count == 309484605
@hokiejer ➜ /workspaces/rustica (iterative_ab_fix) $ target/release/chessica --profile keep-depth-ab --ab-search-depth 9
Running profile script for Keep Depth Alpha-Beta...
Search Depth == 9, Keep Depth == 4
Score == 999592  Move count == 1048953201

Running profile script for Iterative Keep Depth Alpha-Beta... [2]
Search Depth == 9, Keep Depth == 4
i == 1
Score == 975
Move count == 20
i == 2
Score == -586
Move count == 142
i == 3
Score == 880
Move count == 1,924
i == 4
Score == -794
Move count == 24,600
i == 5
Score == 999021
Move count == 102,764
i == 6
Score == -999033
Move count == 3,047,700
i == 7
Score == 999410
Move count == 5,944,444
i == 8
Score == -999134
Move count == 251,388,010
i == 9
Score == 999592
Move count == 731,532,118
Score == 999592

real    3m4.214s
user    2m59.779s
sys     0m0.569s

Running profile script for Iterative Keep Depth Alpha-Beta... [3]
Search Depth == 9, Keep Depth == 4
i == 1
Score == 975
Move count == 20
i == 2
Score == -586
Move count == 142
i == 3
Score == 880
Move count == 1,813
i == 4
Score == -794
Move count == 24,735
i == 5
Score == 999021
Move count == 103,229
i == 6
Score == -999033
Move count == 2,769,731
i == 7
Score == 999410
Move count == 4,940,457
i == 8
Score == -999134
Move count == 240,057,167
i == 9
Score == 999592
Move count == 713,320,125
Score == 999592

real    3m15.402s
user    3m2.852s
sys     0m6.911s

Running profile script for Iterative Keep Depth Alpha-Beta... [4]
Search Depth == 9, Keep Depth == 4
i == 1
Score == 975
Move count == 20
i == 2
Score == -586
Move count == 142
i == 3
Score == 880
Move count == 1,774
i == 4
Score == -794
Move count == 24,843
i == 5
Score == 999021
Move count == 102,262
i == 6
Score == -999033
Move count == 2,764,617
i == 7
Score == 999410
Move count == 4,922,761
i == 8
Score == -999134
Move count == 238,854,045
i == 9
Score == 999592
Move count == 658,076,467
Score == 999592

real    3m26.776s
user    2m58.747s
sys     0m25.097s
