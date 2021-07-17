use chrono::{Date, Datelike, Duration, Local, Weekday};

pub struct RemainingWork {
    date: Date<Local>,
    remaining_time: Duration,
}

impl RemainingWork {
    pub fn new(date: Date<Local>, remaining_time: Duration) -> Self {
        Self {
            date,
            remaining_time: if remaining_time < Duration::zero() {
                Duration::zero()
            } else {
                remaining_time
            },
        }
    }

    pub fn num_working_days(&self) -> u8 {
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

    pub fn time_per_day(&self) -> Duration {
        self.remaining_time / self.num_working_days() as i32
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
    fn test_remaining_work() {
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

        for (date, remaining_time, expected_num_days, expected_time_per_day) in cases {
            let remaining_work = RemainingWork::new(date, remaining_time);

            let num_days = remaining_work.num_working_days();
            let time_per_day = remaining_work.time_per_day();

            assert_eq!(num_days, expected_num_days);
            assert_eq!(time_per_day, expected_time_per_day);
        }
    }
}
