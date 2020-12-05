use super::{report, report2};
use itertools::Itertools;
use rayon::prelude::*;

use chrono::Duration;

use std::collections::HashSet;

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
        let a = *input
            .par_iter()
            .find_any(|e| input.contains(&(2020 - *e)))
            .unwrap();
        report2(
            format!("  {:?}.product = {}", [a, 2020 - a], a * (2020 - a)),
            None,
            duration,
        );
        report("3 elements:", None, duration);
        let mut set = HashSet::new();
        input.iter().for_each(|v| {
            set.insert(v);
        });
        let pt2 = input
            .iter()
            .combinations(2)
            .par_bridge()
            .map(|v| (*v[0], *v[1]))
            .find_any(|&(i, j)| {
                if i + j == 2020 {
                    false
                } else {
                    set.contains(&(2020u32.wrapping_sub(i).wrapping_sub(j)))
                }
            })
            .unwrap();
        report2(
            format!(
                "  {:?}.product = {}",
                [pt2.0, pt2.1, 2020 - pt2.0 - pt2.1],
                pt2.0 * pt2.1 * (2020 - pt2.0 - pt2.1)
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
