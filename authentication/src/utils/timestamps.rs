use prost_types::Timestamp;

pub trait TimestampExt {
    fn to_chrono(&self) -> chrono::DateTime<chrono::Utc>;
    fn from_chrono(dt: chrono::DateTime<chrono::Utc>) -> Self;
}

impl TimestampExt for Timestamp {
    fn to_chrono(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp(self.seconds, self.nanos as u32)
            .unwrap_or(chrono::Utc::now())
    }

    fn from_chrono(dt: chrono::DateTime<chrono::Utc>) -> Self {
        Timestamp {
            seconds: dt.timestamp(),
            nanos: dt.timestamp_subsec_nanos() as i32,
        }
    }
}
