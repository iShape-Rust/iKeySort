use i_key_sort::sort::one_key::OneKeySort;

#[unsafe(no_mangle)]
pub extern "C" fn run_i64_32bit_boundary_regression() -> i32 {
    const BOUNDARY: i64 = 1i64 << 32;

    let mut values = [
        0,
        BOUNDARY - 1,
        BOUNDARY,
        1,
        BOUNDARY / 2,
        BOUNDARY - 2,
        123,
        BOUNDARY - 123,
    ]
    .repeat(128);
    values.reverse();

    let mut expected = values.clone();
    expected.sort_unstable();

    values.sort_by_one_key(false, |value| *value);

    i32::from(values != expected)
}
