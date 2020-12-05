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
                s.replace(&['F', 'L'] as &[char], "0")
                    .replace(&['B', 'R'] as &[char], "1")
            })
            .map(|s| usize::from_str_radix(&s, 2).unwrap())
            .collect::<Vec<usize>>();
        values.par_sort_unstable();
        report(
            format!("max value: {}", values.last().unwrap()),
            None,
            duration,
        );
        let first = *values.first().unwrap();
        let last = *values.last().unwrap();
        let mut set: HashSet<usize> = HashSet::new();
        values.iter().copied().for_each(|v| {
            set.insert(v);
        });
        let idx = (first..=last)
            .par_bridge()
            .find_first(|v| !set.contains(v))
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
