use chrono::DateTime;
use chrono::TimeZone;

use chrono::Utc;

pub struct UnixDayIndex(pub i64);

impl UnixDayIndex {
    pub fn from_timestamp(timestamp: DateTime<Utc>) -> i64 {
        let unix_epoch = Utc.timestamp_opt(0, 0).unwrap(); // Unix epoch: 1970-01-01 00:00:00 UTC
        let duration_since_epoch = timestamp.signed_duration_since(unix_epoch);

        duration_since_epoch.num_days()
    }
}
