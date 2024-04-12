use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_unix_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_true_when_unix_more_than_zero() {
        let timestamp = get_unix_timestamp();
        assert!(timestamp > 0);
    }

    #[test]
    fn should_return_positive_number() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        assert!(timestamp.is_positive())
    }

}