# rustica
Chessica in Rust

## Performance Notes
To allow non-root users to temporarily run `perf`, run this command:
```
sudo sh -c 'echo -1 >/proc/sys/kernel/perf_event_paranoid'
```

I am using `cargo asm` to view generated assembly.
