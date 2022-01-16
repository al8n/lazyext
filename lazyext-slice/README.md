> **Notes**: [WIP] The functions in this crate has not been fully tested.
> Please wait for me to make it 100% tested, or you can give a PR and write test cases for those functions.

<div align="center">
<h1>LazyExt-Slice</h1>
</div>
<div align="center">

Thousands of utility functions for slices and vec.

</div>

## Installation
- std
```toml
[dependencies]
lazeyext-slice = "0.1.0"
```

- no_std (with `alloc` related functions)
```toml
[dependencies]
lazeyext-slice = { version = "0.1.0", default-features = false, features = ["alloc"] }
```

- no_std (without `alloc` related functions)
```toml
[dependencies]
lazeyext-slice = { version = "0.1.0", default-features = false }
```
