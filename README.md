# Clippy Option Map Lint

This is a minimal reproducable example for a suggested improvement to a clippy lint.

1. Clone this repository
   ```
   git clone git@github.com:TimJentzsch/clippy_option_map.git
   cd clippy_option_map
   ```
2. Run clippy
   ```
   cargo clippy
   ```

You will get the following output:

```
warning: manual implementation of `Option::map`
  --> src/main.rs:7:5
   |
7  | /     if let Some(val) = bar {
8  | |         Some(val)
9  | |     } else {
10 | |         None
11 | |     }
   | |_____^ help: try this: `bar.map(|val| val)`
   |
   = note: `#[warn(clippy::manual_map)]` on by default
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#manual_map

warning: `clippy_option_map` (bin "clippy_option_map") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
```

Instead, I would expect clippy to suggest the following implementation:

```rs
fn foo(bar: Option<u8>) -> Option<u8> {
    bar
}
```

Tested with `clippy 0.1.59 (7d6f948 2022-01-04)`.
