use chrono::offset::Local;
use chrono::Duration;

pub mod util;
use util::*;

pub mod solvers;
use solvers::*;

fn main() {
    let solvers: Vec<&dyn Solver> = vec![&c1::Solver];
    prelude();
    let mut duration = Duration::zero();
    for (idx, solver) in solvers.into_iter().enumerate() {
        print_solver(idx, solver);
        let start = Local::now();
        solver.solve(&mut duration);
        let end = Local::now();
        duration = duration + (end - start);
        print_single_duration(duration);
    }
    print_total_duration(duration);
}
