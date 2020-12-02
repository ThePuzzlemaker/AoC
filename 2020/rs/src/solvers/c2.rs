use super::report;

use chrono::Duration;

pub struct Solver;
impl super::Solver for Solver {
    fn solve(&self, duration: &mut Duration) {
        let input = include_str!("../data/2.input");
        let tuples = input.lines().map(|e| {
            let mut split = e.split(' ');
            let mut range = split.next().unwrap().split('-');
            let min = range.next().unwrap().parse::<u8>().unwrap();
            let max = range.next().unwrap().parse::<u8>().unwrap();
            let ch = split.next().unwrap().chars().next().unwrap();
            let pwd = split.next().unwrap();

            (min, max, ch, pwd)
        });
        let pt1 = tuples
            .clone()
            .filter(|&(min, max, ch, pwd)| {
                let count_of_ch = pwd.chars().filter(|c| *c == ch).count() as u8;
                count_of_ch >= min && count_of_ch <= max
            })
            .count();
        report(
            format!("# of valid passwords (via pt1 rules): {}", pt1),
            None,
            duration,
        );
        let pt2 = tuples
            .filter(|&(pos1, pos2, ch, pwd)| {
                let ch1 = pwd.chars().nth(pos1 as usize - 1).unwrap_or('\0');
                let ch2 = pwd.chars().nth(pos2 as usize - 1).unwrap_or('\0');
                (ch1 == ch) != (ch2 == ch)
            })
            .count();
        report(
            format!("# of valid passwords (via pt2 rules): {}", pt2),
            None,
            duration,
        );
    }

    fn name(&self) -> &'static str {
        "Password Philosophy"
    }

    fn num(&self) -> u8 {
        2
    }
}
