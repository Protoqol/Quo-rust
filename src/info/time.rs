use chrono::Local;

pub fn get_time() -> (i64, String) {
    let now = Local::now();
    (
        now.timestamp_millis(),
        now.timestamp_nanos_opt().unwrap_or(0).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_time_epoch_sanity() {
        let (ms, uid) = get_time();
        assert!(ms > 1700000000000); // Sanity check for recent date
        assert!(!uid.is_empty());
    }

    #[test]
    fn test_get_time_monotonicity() {
        let (ms1, _) = get_time();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let (ms2, _) = get_time();
        assert!(ms2 >= ms1);
    }
}
