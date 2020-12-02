use ansi_term::Colour;
use ansi_term::{ANSIString, ANSIStrings};
use chrono::Duration;
use humantime::Duration as HumanDuration;

use super::Solver;

pub(super) fn prelude() {
    let red = Colour::Red.bold();
    let green = Colour::Green.bold();
    let strings: &[ANSIString<'static>] = &[red.paint("="), green.paint("="), red.paint("=")];
    let eqeqeq = ANSIStrings(strings);

    println!(
        "{} {} {} {} {} {} {0}",
        eqeqeq,
        green.paint("Advent"),
        red.paint("of"),
        green.paint("Code"),
        red.paint("2020"),
        green.paint("Solutions")
    );
}

pub(super) fn print_single_duration(duration: Duration) {
    let duration: HumanDuration = duration.to_std().unwrap().into();
    let duration = format!("+{}", duration);
    println!(
        "{}       {}",
        Colour::Fixed(247).bold().paint("│"),
        Colour::Fixed(247).bold().paint("│")
    );
    println!(
        "{}       {} {}",
        Colour::Fixed(247).bold().paint("│"),
        Colour::Fixed(247).bold().paint("╰──"),
        Colour::Green.bold().paint(duration)
    );
}

pub(super) fn print_solver(idx: usize, solver: &dyn Solver) {
    let num = format!("{:02}", solver.num());
    let treech = if idx == 0 {
        Colour::Fixed(247).bold().paint("╭──")
    } else {
        Colour::Fixed(247).bold().paint("├──")
    };
    println!(
        "{} {:02}: {}",
        treech,
        Colour::Green.bold().paint(num),
        Colour::Cyan.bold().paint(solver.name())
    );
}

pub(super) fn print_total_duration(duration: Duration) {
    let duration: HumanDuration = duration.to_std().unwrap().into();
    let duration = format!("total: {}", duration);
    println!("{}", Colour::Fixed(247).bold().paint("│"));
    println!(
        "{} {}",
        Colour::Fixed(247).bold().paint("╰──"),
        Colour::Green.bold().paint(duration)
    );
}
