macro_rules! wrapper {
    ($outer:tt, $inner:tt) => {
        pub mod $outer {
            use aoc::year2024::$inner::{parse, part1 as p1, part2 as p2};
            use std::fmt::Display;

            pub fn part1(input: &str) -> impl Display {
                p1(&parse(input))
            }

            pub fn part2(input: &str) -> impl Display {
                p2(&parse(input))
            }
        }
    };
}

wrapper!(day1, day01);
wrapper!(day2, day02);
wrapper!(day3, day03);
wrapper!(day4, day04);
