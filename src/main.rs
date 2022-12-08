mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod util;

use day2::day_2_star_1_and_2;
use day3::{day_3_star_1, day_3_star_2};
use day4::day_4_star_1_and_2;
use day5::{day_5_star_1, day_5_star_2};

use crate::day1::{day_1_star_1, day_1_star_2};

fn main() {
    day_1_star_1();
    day_1_star_2();

    day_2_star_1_and_2();

    day_3_star_1();
    day_3_star_2();

    day_4_star_1_and_2();

    day_5_star_1();
    day_5_star_2();
}
