use i_key_sort::sort::one_key::OneKeySort;

#[test]
fn sort_i64_across_32_bit_boundary() {
    const BOUNDARY: i64 = 1i64 << 32;

    // On wasm32, the old implementation truncated the full distance to zero
    // while the intermediate distance became `usize::MAX`, escaping the layout.
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

    assert_eq!(values, expected);
}
