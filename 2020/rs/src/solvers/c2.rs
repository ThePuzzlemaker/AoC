use super::report;

use chrono::Duration;

pub struct Solver;
impl super::Solver for Solver {
    fn solve(&self, duration: &mut Duration) {
        let input = include_str!("../data/2.input");
        let tuples = input.lines().map(|e| {
            let mut split = e.split(' ');
            let mut range = split.next().unwrap().split('-');
            let min = range.next().unwrap().parse::<usize>().unwrap();
            let max = range.next().unwrap().parse::<usize>().unwrap();
            let ch = split.next().unwrap().chars().next().unwrap();
            let pwd = split.next().unwrap();

            (min, max, ch, pwd)
        });

        let pt1 = tuples
            .clone()
            .filter(|&(min, max, ch, passwd)| {
                (min..=max).contains(&passwd.chars().filter(|&c| c == ch).count())
            })
            .count();
        report(
            format!("# of valid passwords (via pt1 rules): {}", pt1),
            None,
            duration,
        );

        let pt2 = tuples
            .filter(|&(pos1, pos2, ch, passwd)| {
                let mut iter = passwd.chars();
                (iter.nth(pos1 - 1) == Some(ch)) ^ (iter.nth(pos2 - 1) == Some(ch))
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
