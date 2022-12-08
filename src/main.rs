mod day1;
mod day2;
mod day3;
mod util;

use day2::{day_2_star_1, day_2_star_2};
use day3::day_3_star_1;

use crate::day1::{day1_star_1, day1_star_2};

fn main() {
    day1_star_1();
    day1_star_2();

    day_2_star_1();
    day_2_star_2();

    day_3_star_1();
}
