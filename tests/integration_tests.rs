use polars_row_derive::IterToDataFrame;

#[derive(IterToDataFrame)]
pub struct TestStruct {
    a: i32,
    b: i32,
}

#[test]
fn works() {
    let data = vec![TestStruct { a: 1, b: 2 }, TestStruct { a: 3, b: 4 }];
    let df = data.into_iter().to_dataframe().unwrap();
    assert_eq!(df.shape(), (2, 2));
}

#[test]
fn works_dyn() {
    let df = (0..10)
        .map(|i| TestStruct { a: i, b: i })
        .to_dataframe_dyn()
        .unwrap();
    assert_eq!(df.shape(), (10, 2));
}
