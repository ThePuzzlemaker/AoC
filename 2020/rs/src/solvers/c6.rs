use super::report;
use itertools::Itertools;
use std::iter;

use chrono::Duration;

pub struct Solver;
impl super::Solver for Solver {
    fn solve(&self, duration: &mut Duration) {
        let input = include_str!("../data/6.input");
        let input = input.split("\n\n").collect::<Vec<_>>();
        let res = input
            .iter()
            .map(|&s| s.chars().chain(iter::once('\n')).unique().count() - 1)
            .sum::<usize>();
        report(
            format!(
                "sum of # of questions answered per group (w/o duplicates): {}",
                res
            ),
            None,
            duration,
        );
        let res = input
            .iter()
            .map(|s| {
                ('a'..='z')
                    .filter(|&c| s.split('\n').all(|e| e.contains(c)))
                    .count()
            })
            .sum::<usize>();
        report(
            format!(
                "sum of # of questions answered by every member in group: {}",
                res
            ),
            None,
            duration,
        );
    }

    fn name(&self) -> &'static str {
        "Custom Customs"
    }

    fn num(&self) -> u8 {
        6
    }
}
