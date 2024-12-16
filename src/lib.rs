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
wrapper!(day5, day05);
wrapper!(day6, day06);
wrapper!(day7, day07);
wrapper!(day8, day08);
wrapper!(day9, day09);
wrapper!(day10, day10);
wrapper!(day11, day11);
wrapper!(day12, day12);
wrapper!(day13, day13);
wrapper!(day14, day14);
wrapper!(day15, day15);
wrapper!(day16, day16);
