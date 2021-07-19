use chrono::{Date, Datelike, Duration, Local, Month, TimeZone};
use num_traits::cast::FromPrimitive;

use crate::{remaining_work::RemainingWork, report::Report, util};

pub struct Timesheet {
    entries: Vec<(Date<Local>, Duration)>,
}

impl Timesheet {
    pub fn parse_report(report: &Report) -> Self {
        let mut entries = Vec::new();
        for line in report.0.split("\n") {
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

    pub fn generate_report(&self) -> Report {
        let mut lines = Vec::new();
        let mut total = Duration::seconds(0);
        for (i, &(date, duration)) in self.entries.iter().enumerate() {
            total = total + duration;
            lines.push(format!(
                "{} {}",
                format_date(&date),
                util::format_duration(duration)
            ));
            if i == self.entries.len() - 1 || date.month() != self.entries[i + 1].0.month() {
                lines.push(format!(
                    "Total for {} {} {}\n",
                    Month::from_u32(date.month()).unwrap().name(),
                    date.year(),
                    util::format_duration(total)
                ));
                total = Duration::seconds(0);
            }
        }
        return Report(lines.join("\n"));
    }

    pub fn add_hours(&mut self, date: &Date<Local>, duration: &Duration) {
        match self.binary_search(date) {
            Ok(i) => self.entries[i].1 = self.entries[i].1 + *duration,
            Err(i) => self.entries.insert(i, (date.clone(), duration.clone())),
        };
    }

    pub fn get_hours(&self, date: &Date<Local>) -> Duration {
        match self.binary_search(date) {
            Ok(i) => self.entries[i].1,
            Err(_) => Duration::seconds(0),
        }
    }

    pub fn remaining_work(&self) -> Option<RemainingWork> {
        self.entries.last().map(|&(last, _)| {
            RemainingWork::new(
                last,
                Duration::hours(160) - self.hours_worked_in_month(last.month()),
            )
        })
    }

    fn binary_search(&self, date: &Date<Local>) -> Result<usize, usize> {
        self.entries.binary_search_by(|&(d, _)| d.cmp(&date))
    }

    fn hours_worked_in_month(&self, month: u32) -> Duration {
        self.entries
            .iter()
            .rev()
            .take_while(|e| e.0.month() == month)
            .fold(Duration::zero(), |acc, e| acc + e.1)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_report() {
        let timesheet = Timesheet::parse_report(&Report("".to_owned()));

        assert_eq!(timesheet.entries.len(), 0);
    }

    #[test]
    fn test_timesheet_reporting() {
        let cases = vec![
            (
                "
03.02.2021 05:00:12
Total for February 2021 05:00:12
",
                Timesheet {
                    entries: vec![(
                        Local.ymd(2021, 2, 3),
                        Duration::hours(5) + Duration::seconds(12),
                    )],
                },
            ),
            (
                "
01.01.2021 07:00:12
Total for January 2021 07:00:12

01.02.2021 07:00:12
02.02.2021 09:17:12
03.02.2021 05:00:12
11.02.2021 05:01:13
Total for February 2021 26:18:49
",
                Timesheet {
                    entries: vec![
                        (
                            Local.ymd(2021, 1, 1),
                            Duration::hours(7) + Duration::seconds(12),
                        ),
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
01.01.2021 07:00:12
04.01.2021 07:00:12
Total for January 2021 14:00:24

01.02.2021 07:00:12
02.02.2021 09:17:12
03.02.2021 05:00:12
11.02.2021 05:01:13
Total for February 2021 26:18:49
",
                Timesheet {
                    entries: vec![
                        (
                            Local.ymd(2021, 1, 1),
                            Duration::hours(7) + Duration::seconds(12),
                        ),
                        (
                            Local.ymd(2021, 1, 4),
                            Duration::hours(7) + Duration::seconds(12),
                        ),
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
01.02.2021 07:00:12
02.02.2021 09:17:12
03.02.2021 05:00:12
11.02.2021 05:01:13
Total for February 2021 26:18:49
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

        for (i, (report, expected_timesheet)) in cases.into_iter().enumerate() {
            println!("{}", i);

            let timesheet = Timesheet::parse_report(&Report(report.to_owned()));

            assert_eq!(timesheet.entries, expected_timesheet.entries);
            assert_eq!(timesheet.generate_report().0.trim(), report.trim());
        }
    }

    #[test]
    fn test_timesheet_add_hours_existing_date() {
        let mut timesheet = create_sample_timesheet();

        timesheet.add_hours(
            &Local.ymd(2021, 3, 11),
            &(Duration::hours(2) + Duration::minutes(12)),
        );

        let report = timesheet.generate_report();
        assert_eq!(
            report.0,
            "01.03.2021 01:00:00
02.03.2021 01:14:00
03.03.2021 02:00:01
11.03.2021 03:13:00
31.03.2021 01:01:00
Total for March 2021 08:28:01
"
        );
    }

    #[test]
    fn test_timesheet_add_hours_new_date() {
        let mut timesheet = create_sample_timesheet();

        timesheet.add_hours(
            &Local.ymd(2021, 3, 12),
            &(Duration::hours(2) + Duration::minutes(12)),
        );

        let report = timesheet.generate_report();
        assert_eq!(
            report.0,
            "01.03.2021 01:00:00
02.03.2021 01:14:00
03.03.2021 02:00:01
11.03.2021 01:01:00
12.03.2021 02:12:00
31.03.2021 01:01:00
Total for March 2021 08:28:01
"
        );
    }

    fn create_sample_timesheet() -> Timesheet {
        Timesheet::parse_report(&Report(
            "
01.03.2021 01:00:00
02.03.2021 01:14:00
03.03.2021 02:00:01
11.03.2021 01:01:00
31.03.2021 01:01:00
Total for March 2021 06:16:01
                "
            .to_owned(),
        ))
    }
}
