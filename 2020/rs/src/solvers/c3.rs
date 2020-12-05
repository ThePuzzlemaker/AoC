use super::{report, report2};
use chrono::Duration;

use std::collections::HashSet;

pub struct Solver;
impl super::Solver for Solver {
    fn solve(&self, duration: &mut Duration) {
        let input = include_str!("../data/3.input");
        let llen = input.lines().next().unwrap().len();
        let nlines = input.lines().count();
        let tree_indices = input
            .char_indices()
            .filter(|&(_, c)| c == '#')
            .map(|(i, _)| i)
            .collect::<HashSet<_>>();

        report(format!("line length: {}", llen), None, duration);

        let ntrees_per_slope = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|&slope| {
                let mut found = 0;
                let mut row = 0;
                let mut col = 0;
                while row <= nlines {
                    if tree_indices.contains(&(row * (llen + 1) + (col % llen))) {
                        found += 1;
                    }
                    col += slope.0;
                    row += slope.1;
                }
                report(
                    format!(
                        "found # of trees for slope ({}, {}): {}",
                        slope.0, slope.1, found
                    ),
                    None,
                    duration,
                );
                found
            })
            .collect::<Vec<_>>();

        let res = ntrees_per_slope.iter().product::<usize>();
        report("product of # of trees found:", None, duration);
        report2(
            format!("  {:?}.product = {}", ntrees_per_slope, res),
            None,
            duration,
        );
    }

    fn name(&self) -> &'static str {
        "Toboggan Trajectory"
    }

    fn num(&self) -> u8 {
        3
    }
}
