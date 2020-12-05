use chrono::Duration;

pub trait Solver {
    fn num(&self) -> u8;
    fn name(&self) -> &'static str;
    fn solve(&self, duration: &mut Duration);
}

use ansi_term::Colour;
use ansi_term::Style;
use chrono::offset::Local;

pub fn report(output: impl AsRef<str>, style: Option<Style>, duration: &mut Duration) {
    let start = Local::now();
    println!(
        "{}       {} {}",
        Colour::Fixed(247).bold().paint("│"),
        Colour::Fixed(247).bold().paint("├──"),
        style
            .unwrap_or_else(|| Colour::White.bold())
            .paint(output.as_ref())
    );
    let end = Local::now();
    // Ignore the time it took to print this message.
    *duration = *duration - (end - start);
}

pub fn report2(output: impl AsRef<str>, style: Option<Style>, duration: &mut Duration) {
    let start = Local::now();
    println!(
        "{}       {} {}",
        Colour::Fixed(247).bold().paint("│"),
        Colour::Fixed(247).bold().paint("│  "),
        style
            .unwrap_or_else(|| Colour::White.bold())
            .paint(output.as_ref())
    );
    let end = Local::now();
    // Ignore the time it took to print this message.
    *duration = *duration - (end - start);
}

pub mod c1;
pub mod c2;
pub mod c3;
pub mod c4;
