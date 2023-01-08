# Rust

## `pub(path)` conjection

> see [official reference](https://doc.rust-lang.org/reference/visibility-and-privacy.htmlpubin-path-pubcrate-pubsuper-and-pubself), [related error](https://doc.rust-lang.org/error-index.html#E0704), [my playground example](https://github.com/ah-y/playground/blob/master/rust/elseeee/src/main.rs)

Since 2018 Edition, `path` for `pub(path)` must start with crate, super.

## p formatting

> see [more detail](https://doc.rust-lang.org/core/fmt/trait.Pointer.html)

show pointer's memory location by using p formatting.

```rust
let x = &42;
let address = format!("{x:p}"); // this produces something like '0x7f06092ac6d0'
```

---

# Vim

## Mode as a Motion

In Normal mode, typing `v` enter visual mode.
Then type `iw` selects word the cursor is currently on.
Other example, typing `vi"` selects inner " .. ".
