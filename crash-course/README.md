# Rust Programming Crash Course: the code

The program from the **Rust Programming Crash Course** on Honest Engineering. It reads a messy `bookmarks.txt`, skips the lines it can't parse, and groups the rest by tag.

Rust 1.99.1, edition 2024, pinned in `rust-toolchain.toml`, so the compiler messages you see match the video.

```
cd crash-course
cargo run
cargo test
```

`src/` is the finished program. `steps/b01.rs` to `steps/b15.rs` are the whole `main.rs` at each step of the video. To run one, copy it over `src/main.rs` (and delete `src/bookmark.rs` first, because the module split only happens at the end).
