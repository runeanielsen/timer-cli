use std::time::SystemTime;

pub trait UnixEpoch {
    fn unix_epoch(&self) -> u64;
}

impl UnixEpoch for SystemTime {
    fn unix_epoch(&self) -> u64 {
        self.duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn end_time_unix_epoch_is_bigger_than_default() {
        assert!(SystemTime::now().unix_epoch() > 0);
    }
}
