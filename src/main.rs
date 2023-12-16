extern crate num;

use num::Complex;

use mylib::main_single;

fn main() {
    let bounds = (1000, 750);
    let upper_left = Complex { re: -1.20, im: 0.35 };
    let lower_right = Complex { re: -1.0, im: 0.20 };

    main_single(bounds, upper_left, lower_right);
}
