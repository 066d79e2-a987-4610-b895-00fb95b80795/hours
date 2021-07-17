use chrono::{Date, Datelike, Duration, Local, Weekday};

pub struct WorkDays {
    date: Date<Local>,
    remaining_work: Duration,
}

impl WorkDays {
    pub fn new(date: Date<Local>, remaining_work: Duration) -> Self {
        Self {
            date,
            remaining_work,
        }
    }

    pub fn remaining_days_in_month(&self) -> u8 {
        let month = self.date.month();
        let mut date = self.date;
        let mut remaining_days = 0;
        while date.month() == month {
            if is_working_day(date.weekday()) {
                remaining_days += 1;
            }
            date = date.succ();
        }
        remaining_days
    }

    pub fn remaining_work_per_day(&self) -> Duration {
        self.remaining_work / self.remaining_days_in_month() as i32
    }
}

fn is_working_day(weekday: Weekday) -> bool {
    match weekday {
        Weekday::Sat | Weekday::Sun => false,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn test_remaining_days() {
        let cases = vec![
            (
                Local.ymd(2021, 7, 17),
                Duration::hours(10),
                10,
                Duration::hours(1),
            ),
            (
                Local.ymd(2021, 8, 20),
                Duration::hours(4),
                8,
                Duration::minutes(30),
            ),
        ];

        for (date, work, expected_remaining_days, expected_remaining_work_per_day) in cases {
            let work_days = WorkDays::new(date, work);

            let remaining_days = work_days.remaining_days_in_month();
            let remaining_work = work_days.remaining_work_per_day();

            assert_eq!(remaining_days, expected_remaining_days);
            assert_eq!(remaining_work, expected_remaining_work_per_day);
        }
    }
}
