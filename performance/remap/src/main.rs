use i_key_sort::sort::one_key::OneKeySort;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct LineRange {
    min: i32,
    max: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct XDig {
    x: i32,
    range: LineRange,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct YDig {
    y: i32,
    range: LineRange,
}

fn main() {
    let mut x_vec = vec![
        XDig {
            x: 6,
            range: LineRange { min: 1, max: 3 },
        },
        XDig {
            x: 3,
            range: LineRange { min: 4, max: 6 },
        },
        XDig {
            x: 8,
            range: LineRange { min: 7, max: 9 },
        },
        XDig {
            x: 2,
            range: LineRange { min: 10, max: 12 },
        },
    ];

    let mut y_vec = vec![
        YDig {
            y: 2,
            range: LineRange { min: 13, max: 15 },
        },
        YDig {
            y: 5,
            range: LineRange { min: 16, max: 18 },
        },
        YDig {
            y: 1,
            range: LineRange { min: 19, max: 21 },
        },
        YDig {
            y: 3,
            range: LineRange { min: 22, max: 24 },
        },
    ];

    let mut x_buf = Vec::new();
    x_vec.sort_by_one_key_and_buffer(false, &mut x_buf, |d| d.x);

    let mut y_buf = Vec::new();
    y_vec.sort_by_one_key_and_buffer(false, &mut y_buf, |d| d.y);

    assert!(x_vec.windows(2).all(|w| w[0].x <= w[1].x));
    assert!(y_vec.windows(2).all(|w| w[0].y <= w[1].y));
    println!("{x_vec:?}");
    println!("{y_vec:?}");
}
