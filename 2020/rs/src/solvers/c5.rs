use super::report;
use rayon::prelude::*;

use std::collections::HashSet;

use chrono::Duration;

pub struct Solver;
impl super::Solver for Solver {
    fn solve(&self, duration: &mut Duration) {
        let input = include_str!("../data/5.input");
        let mut values = input
            .par_split('\n')
            .map(|s| {
                s.chars()
                    .fold(0, |p, c| p * 2 + (c == 'B' || c == 'R') as usize)
            })
            .collect::<Vec<usize>>();
        values.par_sort_unstable();
        let first = *values.first().unwrap();
        let last = *values.last().unwrap();
        report(format!("max value: {}", last), None, duration);
        let mut set: HashSet<usize> = HashSet::new();
        set.extend(values.into_iter());
        let idx = (first..=last)
            .par_bridge()
            .find_any(|v| !set.contains(v))
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
