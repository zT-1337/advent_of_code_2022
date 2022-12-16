mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod util;

use day10::day_10_star_1;
use day2::day_2_star_1_and_2;
use day3::{day_3_star_1, day_3_star_2};
use day4::day_4_star_1_and_2;
use day5::{day_5_star_1, day_5_star_2};
use day6::day_6_star_1_and_2;
use day7::day_7_star_1;
use day8::{day_8_star_1, day_8_star_2};
use day9::day_9_star_1_and_2;

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

    day_6_star_1_and_2();

    day_7_star_1();

    day_8_star_1();
    day_8_star_2();

    day_9_star_1_and_2();

    day_10_star_1();
}
