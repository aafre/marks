# marks

A bookmark manager built step by step in the Honest Engineering **Rust Build-Along** series. It starts as a std-only command-line tool and grows into a web service.

Each episode ends on a tag (`ep01`, `ep02`, …), so you can check out exactly where the video ended:

```
git checkout ep01
cargo run -- add https://doc.rust-lang.org/book/
cargo run -- list
```

The toolchain is pinned in `rust-toolchain.toml` (Rust 1.99.1, edition 2024), so the compiler messages you see match the video.

This is an independent tutorial project. It isn't affiliated with or endorsed by the Rust Project or the Rust Foundation.
