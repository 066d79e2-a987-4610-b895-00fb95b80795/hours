use chrono::{Date, Datelike, Duration, Local, Month, TimeZone};
use num_traits::cast::FromPrimitive;

use crate::util;

pub struct Timesheet {
    entries: Vec<(Date<Local>, Duration)>,
}

impl Timesheet {
    pub fn parse_report(report: &str) -> Self {
        let mut entries = Vec::new();
        for line in report.split("\n") {
            let line = line.trim();
            if line == "" || line.to_lowercase().starts_with("total") {
                continue;
            }
            let mut pieces = line.split(" ");
            let date = parse_date(pieces.next().unwrap());
            let duration = util::parse_duration(pieces.next().unwrap());
            entries.push((date, duration));
        }
        Self { entries }
    }

    pub fn generate_report(&self) -> String {
        let mut lines = Vec::new();
        let mut total = Duration::seconds(0);
        for &(date, duration) in self.entries.iter() {
            total = total + duration;
            lines.push(format!(
                "{} {}",
                format_date(&date),
                util::format_duration(duration)
            ));
            if is_last_day_of_month(&date) {
                lines.push(format!(
                    "Total for {} {} {}",
                    Month::from_u32(date.month()).unwrap().name(),
                    date.year(),
                    util::format_duration(total)
                ));
                total = Duration::seconds(0);
            }
        }
        return lines.join("\n");
    }

    pub fn add_hours(date: Date<Local>, duration: Duration) {}
}

fn is_last_day_of_month(date: &Date<Local>) -> bool {
    let succ = date.succ();
    succ.month() != date.month()
}

fn parse_date(s: &str) -> Date<Local> {
    let mut pieces = s.split(".");
    let day: u32 = pieces.next().unwrap().parse().unwrap();
    let month: u32 = pieces.next().unwrap().parse().unwrap();
    let year: i32 = pieces.next().unwrap().parse().unwrap();
    Local.ymd(year, month, day)
}

fn format_date(d: &Date<Local>) -> String {
    format!("{:0>2}.{:0>2}.{}", d.day(), d.month(), d.year())
}

impl ToString for Timesheet {
    fn to_string(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timesheet_reporting() {
        let cases = vec![
            (
                "
01.02.2021 07:00:12
02.02.2021 09:17:12
03.02.2021 05:00:12
11.02.2021 05:01:13
",
                Timesheet {
                    entries: vec![
                        (
                            Local.ymd(2021, 2, 1),
                            Duration::hours(7) + Duration::seconds(12),
                        ),
                        (
                            Local.ymd(2021, 2, 2),
                            Duration::hours(9) + Duration::minutes(17) + Duration::seconds(12),
                        ),
                        (
                            Local.ymd(2021, 2, 3),
                            Duration::hours(5) + Duration::seconds(12),
                        ),
                        (
                            Local.ymd(2021, 2, 11),
                            Duration::hours(5) + Duration::minutes(1) + Duration::seconds(13),
                        ),
                    ],
                },
            ),
            (
                "
01.03.2021 01:00:00
02.03.2021 01:14:00
03.03.2021 02:00:01
11.03.2021 01:01:00
31.03.2021 01:01:00
Total for March 2021 06:16:01
",
                Timesheet {
                    entries: vec![
                        (Local.ymd(2021, 3, 1), Duration::hours(1)),
                        (
                            Local.ymd(2021, 3, 2),
                            Duration::hours(1) + Duration::minutes(14),
                        ),
                        (
                            Local.ymd(2021, 3, 3),
                            Duration::hours(2) + Duration::seconds(1),
                        ),
                        (
                            Local.ymd(2021, 3, 11),
                            Duration::hours(1) + Duration::minutes(1),
                        ),
                        (
                            Local.ymd(2021, 3, 31),
                            Duration::hours(1) + Duration::minutes(1),
                        ),
                    ],
                },
            ),
        ];

        for (report, expected_timesheet) in cases.into_iter() {
            let timesheet = Timesheet::parse_report(report);

            assert_eq!(timesheet.entries, expected_timesheet.entries);
            assert_eq!(timesheet.generate_report().trim(), report.trim());
        }
    }
}
