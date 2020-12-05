use super::report;
use rayon::prelude::*;

use chrono::Duration;

pub struct Solver;
impl super::Solver for Solver {
    fn solve(&self, duration: &mut Duration) {
        let input = include_str!("../data/4.input");
        let records = input
            .split("\n\n")
            .map(Record::from_record_str)
            .collect::<Vec<Record>>();
        let pt1 = records
            .par_iter()
            .copied()
            .filter(Record::verify_fields_present)
            .count();
        report(
            format!("# of passports w/ all expected fields: {}", pt1),
            None,
            duration,
        );
        let pt2 = records
            .par_iter()
            .copied()
            .filter(Record::verify_fields_present)
            .filter(Record::verify_field_values)
            .count();
        report(
            format!("# of passports w/ all valid fields: {}", pt2),
            None,
            duration,
        );
    }

    fn name(&self) -> &'static str {
        "Passport Processing"
    }

    fn num(&self) -> u8 {
        4
    }
}

type OptionalStr = Option<&'static str>;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Record {
    byr: OptionalStr,
    iyr: OptionalStr,
    eyr: OptionalStr,
    hgt: OptionalStr,
    hcl: OptionalStr,
    ecl: OptionalStr,
    pid: OptionalStr,
    cid: OptionalStr,
}

impl Record {
    fn from_record_str(s: &'static str) -> Self {
        let fields = s.split(|c| c == '\n' || c == ' ');
        let mut rec = Record::default();
        for field in fields {
            let mut split = field.split(':');
            let name = split.next().unwrap();
            let value = split.next().unwrap();
            match name {
                "byr" => rec.byr = Some(value),
                "iyr" => rec.iyr = Some(value),
                "eyr" => rec.eyr = Some(value),
                "hgt" => rec.hgt = Some(value),
                "hcl" => rec.hcl = Some(value),
                "ecl" => rec.ecl = Some(value),
                "pid" => rec.pid = Some(value),
                "cid" => rec.cid = Some(value),
                _ => panic!("unexpected field"),
            }
        }
        rec
    }

    fn verify_fields_present(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn verify_field_values(&self) -> bool {
        self.verify_yrs()
            && self.verify_hgt()
            && self.verify_hcl()
            && self.verify_ecl()
            && self.verify_pid()
    }

    fn verify_yrs(&self) -> bool {
        let byr = self.byr.unwrap().parse::<usize>();
        if byr.is_err() {
            return false;
        }
        let byr = byr.unwrap();

        let iyr = self.iyr.unwrap().parse::<usize>();
        if iyr.is_err() {
            return false;
        }
        let iyr = iyr.unwrap();

        let eyr = self.eyr.unwrap().parse::<usize>();
        if eyr.is_err() {
            return false;
        }
        let eyr = eyr.unwrap();

        byr >= 1920 && byr <= 2002 && iyr >= 2010 && iyr <= 2020 && eyr >= 2020 && eyr <= 2030
    }

    fn verify_hgt(&self) -> bool {
        let hgt = self.hgt.unwrap();
        let hgti = if hgt.par_chars().count() >= 2 {
            let mut vec = hgt.chars().collect::<Vec<char>>();
            vec.pop();
            vec.pop();
            let hgti = vec.iter().collect::<String>().parse::<usize>();
            if hgti.is_err() {
                return false;
            }
            hgti.unwrap()
        } else {
            return false;
        };

        if hgt.ends_with("cm") {
            150 <= hgti && hgti <= 193
        } else if hgt.ends_with("in") {
            59 <= hgti && hgti <= 76
        } else {
            false
        }
    }

    fn verify_hcl(&self) -> bool {
        let hcl = self.hcl.unwrap();
        hcl.starts_with('#')
            && hcl
                .chars()
                .skip(1)
                .par_bridge()
                .all(|ch| ch.is_ascii_hexdigit())
            && hcl.par_chars().count() == 7
    }

    fn verify_ecl(&self) -> bool {
        let ecl = self.ecl.unwrap();
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl)
    }

    fn verify_pid(&self) -> bool {
        let pid = self.pid.unwrap();
        pid.par_chars().all(|ch| ch.is_ascii_digit()) && pid.par_chars().count() == 9
    }
}
