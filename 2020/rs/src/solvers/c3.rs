use super::{report, report2};
use rayon::prelude::*;

use chrono::Duration;

pub struct Solver;
impl super::Solver for Solver {
    fn solve(&self, duration: &mut Duration) {
        let input = include_str!("../data/3.input");
        let llen = input.lines().next().unwrap().len();
        let nlines = input.par_lines().count();
        let tree_indices = input
            .par_char_indices()
            .filter(|&(_, c)| c == '#')
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        report(format!("line length: {}", llen), None, duration);
        let ntreeses = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .into_par_iter()
            .map(|s| {
                let mut dur = Duration::zero();
                calc(s, &tree_indices, nlines, llen, &mut dur)
            })
            .collect::<Vec<_>>();
        let res = ntreeses.iter().product::<usize>();
        report("product of # of trees found:", None, duration);
        report2(
            format!("  {:?}.product = {}", ntreeses, res),
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

fn row_col_to_char_idx(llen: usize, row: usize, col: usize) -> usize {
    let col = col % llen;
    row * (llen + 1) + col
}

fn is_tree(llen: usize, row: usize, col: usize, tree_indices: &[usize]) -> bool {
    tree_indices.contains(&row_col_to_char_idx(llen, row, col))
}

fn calc(
    slope: (usize, usize),
    tree_indices: &[usize],
    nlines: usize,
    llen: usize,
    duration: &mut Duration,
) -> usize {
    let mut found = 0;
    let mut row = 0;
    let mut col = 0;
    while row <= nlines {
        if is_tree(llen, row, col, tree_indices) {
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
}
