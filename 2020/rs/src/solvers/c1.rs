use super::{report, report2};
use itertools::Itertools;

use chrono::Duration;

pub struct Solver;
impl super::Solver for Solver {
    fn solve(&self, duration: &mut Duration) {
        let input = include_str!("../data/1.input");
        let mut input = input
            .lines()
            .map(str::parse::<u32>)
            .map(Result::unwrap)
            .collect::<Vec<u32>>();
        // Sorting the list and dedup-ing it drastically improves performance.
        input.sort_unstable();
        input.dedup();

        report("2 elements:", None, duration);
        let comb2 = input
            .iter()
            .combinations(2)
            .find(|e| e.iter().copied().sum::<u32>() == 2020)
            .unwrap();
        report2(
            format!(
                "  {:?}.product = {}",
                comb2,
                comb2.iter().copied().product::<u32>()
            ),
            None,
            duration,
        );
        report("3 elements:", None, duration);
        let comb3 = input
            .iter()
            .combinations(3)
            .find(|e| e.iter().copied().sum::<u32>() == 2020)
            .unwrap();
        report2(
            format!(
                "  {:?}.product = {}",
                comb3,
                comb3.iter().copied().product::<u32>()
            ),
            None,
            duration,
        );
    }

    fn name(&self) -> &'static str {
        "Report Repair"
    }

    fn num(&self) -> u8 {
        1
    }
}
