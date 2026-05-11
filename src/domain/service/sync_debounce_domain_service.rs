use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// 同期リクエストのデバウンス制御を行うドメインサービス
pub struct SyncDebounceDomainService {
    last_request_time: Arc<Mutex<Option<Instant>>>,
    debounce_duration: Duration,
}

impl SyncDebounceDomainService {
    pub fn new(debounce_ms: u64) -> Self {
        Self {
            last_request_time: Arc::new(Mutex::new(None)),
            debounce_duration: Duration::from_millis(debounce_ms),
        }
    }

    /// リクエストを実行すべきかどうかを判定する
    pub fn should_execute(&self) -> bool {
        let mut last_time = self.last_request_time.lock().unwrap();
        let now = Instant::now();

        if matches!(*last_time, Some(time) if now.duration_since(time) < self.debounce_duration) {
            return false;
        }

        *last_time = Some(now);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_debounce_logic() {
        let service = SyncDebounceDomainService::new(100);

        // 1回目は成功
        assert!(service.should_execute());

        // 直後の2回目は失敗
        assert!(!service.should_execute());

        // 150ms 待機後は成功
        thread::sleep(Duration::from_millis(150));
        assert!(service.should_execute());
    }
}
