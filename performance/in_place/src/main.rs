mod exp;
mod geom;
mod solver;
mod util;

// target/release/in_place --test 0 --count 100
fn main() {
    #[cfg(debug_assertions)]
    {
        debug_run();
    }

    #[cfg(not(debug_assertions))]
    {
        release_run();
    }
}

#[cfg(debug_assertions)]
fn debug_run() {
    exp::checkerboard::CheckerboardTest::new(10).run_custom();
}

#[cfg(not(debug_assertions))]
fn release_run() {
    use crate::util::args::EnvArgs;

    let args = EnvArgs::new();
    let test = args.get_usize("test");
    let count = args.get_usize("count");
    match test {
        0 => test_0(count),
        1 => test_1(count),
        2 => test_2(count),
        3 => test_3(count),
        _ => {
            panic!("No test found")
        }
    }
}

#[allow(dead_code)]
fn test_0(n: usize) {
    exp::checkerboard::CheckerboardTest::new(n).run_all();
}

#[allow(dead_code)]
fn test_1(n: usize) {
    exp::random::RandomTest::new(n).run_all();
}

#[allow(dead_code)]
fn test_2(n: usize) {
    exp::circle::CircleTest::new(n).run_all();
}

#[allow(dead_code)]
fn test_3(n: usize) {
    exp::spiral::SpiralTest::new(n).run_all();
}