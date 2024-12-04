macro_rules! wrapper {
    ($outer:tt, $inner:tt) => {
        pub mod $outer {
            use aoc::year2024::$inner::{parse, part1 as p1, part2 as p2};

            #[inline]
            pub fn part1(input: &str) -> String {
                p1(&parse(input)).to_string()
            }

            #[inline]
            pub fn part2(input: &str) -> String {
                p2(&parse(input)).to_string()
            }
        }
    };
}

wrapper!(day1, day01);
wrapper!(day2, day02);
wrapper!(day3, day03);
wrapper!(day4, day04);
