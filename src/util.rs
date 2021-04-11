use chrono::Duration;

pub fn parse_duration(s: &str) -> Duration {
    let mut pieces = s.split(":");
    let hours: i64 = pieces.next().unwrap().parse().unwrap();
    let mins: i64 = pieces.next().unwrap().parse().unwrap();
    let secs: i64 = pieces.next().unwrap().parse().unwrap();
    Duration::hours(hours) + Duration::minutes(mins) + Duration::seconds(secs)
}

pub fn format_duration(duration: Duration) -> String {
    let mut secs = duration.num_seconds();
    let mut mins = secs / 60;
    secs -= mins * 60;
    let hours = mins / 60;
    mins -= hours * 60;
    format!("{:0>2}:{:0>2}:{:0>2}", hours, mins, secs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = [
            (Duration::seconds(12), "00:00:12"),
            (Duration::seconds(60), "00:01:00"),
            (Duration::seconds(61), "00:01:01"),
            (Duration::minutes(2) + Duration::seconds(11), "00:02:11"),
            (Duration::hours(1) + Duration::minutes(21), "01:21:00"),
            (
                Duration::hours(1) + Duration::minutes(0) + Duration::seconds(1),
                "01:00:01",
            ),
            (
                Duration::hours(1) + Duration::minutes(0) + Duration::seconds(59),
                "01:00:59",
            ),
        ];

        for &(duration, string) in cases.iter() {
            assert_eq!(format_duration(duration), string);
            assert_eq!(parse_duration(string), duration);
        }
    }
}
