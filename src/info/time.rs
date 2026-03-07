use chrono::Utc;

pub fn get_time() -> (i64, String) {
    let now = Utc::now();
    ( 
        now.timestamp_millis(),
        now.timestamp_nanos_opt().unwrap_or(0).to_string(),
    )
}
