use super::report;
use itertools::Itertools;

use chrono::Duration;

pub struct Solver;
impl super::Solver for Solver {
    fn solve(&self, duration: &mut Duration) {
        let input = include_str!("../data/5.input");
        let mut values = input
            .split('\n')
            .map(|s| {
                s.chars()
                    .fold(0, |p, c| p * 2 + (c == 'B' || c == 'R') as usize)
            })
            .collect::<Vec<usize>>();
        values.sort_unstable();
        let last = *values.iter().max().unwrap();
        report(format!("max value: {}", last), None, duration);
        let idx = values
            .into_iter()
            .tuple_windows()
            .find(|&(prev, i)| i != prev + 1)
            .map(|(prev, _)| prev + 1)
            .unwrap();
        report(format!("my seat: {}", idx), None, duration);
    }

    fn name(&self) -> &'static str {
        "Binary Boarding"
    }

    fn num(&self) -> u8 {
        5
    }
}
