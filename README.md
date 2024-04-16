# polars-row-derive

This is a simple crate that allows you derive a custom trait to convert
an iterator over your structs into a `DataFrame` from the `polars` crate.

## Example

```rust
use polars_row_derive::IterToDataFrame;

#[derive(IterToDataFrame)]
pub struct TestStruct {
    a: i32,
    b: i32,
}

// Dynamic size iterator
let df = (0..10)
    .filter(|i| i % 2 == 0)
    .map(|i| TestStruct { a: i, b: i })
    .to_dataframe_dyn()
    .unwrap();

assert_eq!(df.shape(), (5, 2));

// Fixed size iterator
let df = (0..10)
    .map(|i| TestStruct { a: i, b: i })
    .to_dataframe()
    .unwrap();

assert_eq!(df.shape(), (10, 2));
```
