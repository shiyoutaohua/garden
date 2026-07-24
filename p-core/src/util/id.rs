use ferroid::{
    generator::AtomicSnowflakeGenerator,
    id::{SnowflakeTwitterId, ULID},
    time::{MonotonicClock, TWITTER_EPOCH},
};
use std::sync::{
    LazyLock,
    atomic::{AtomicU64, Ordering},
};

static ID: AtomicU64 = AtomicU64::new(1);
pub struct SimpleIdGenerator;
impl SimpleIdGenerator {
    pub fn next() -> u64 {
        ID.fetch_add(1, Ordering::Relaxed)
    }
}

pub struct SnowflakeGenerator;
impl SnowflakeGenerator {
    pub fn next() -> u64 {
        static SNOWFLAKE: LazyLock<AtomicSnowflakeGenerator<SnowflakeTwitterId, MonotonicClock>> =
            LazyLock::new(|| {
                let machine_id = 0;
                AtomicSnowflakeGenerator::new(machine_id, MonotonicClock::with_epoch(TWITTER_EPOCH))
            });
        let id = SNOWFLAKE.next_id(|_| std::hint::spin_loop());
        id.to_raw()
    }
}

pub struct UuidGenerator;
impl UuidGenerator {
    pub fn next_v4() -> String {
        uuid::Uuid::new_v4().simple().to_string()
    }

    pub fn next_v7() -> String {
        uuid::Uuid::now_v7().simple().to_string()
    }
}

pub struct UlidGenerator;
impl UlidGenerator {
    pub fn next() -> String {
        let mut id = ULID::now().to_string();
        id.make_ascii_lowercase();
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_id() {
        assert_eq!(SimpleIdGenerator::next(), 1);
        assert_eq!(SimpleIdGenerator::next(), 2);
    }

    #[test]
    fn test_snowflake() {
        let a = SnowflakeGenerator::next();
        let b = SnowflakeGenerator::next();
        assert!(a < b);
    }

    #[test]
    fn test_uuid() {
        let a = UuidGenerator::next_v7();
        let b = UuidGenerator::next_v7();
        assert!(a < b);
    }

    #[test]
    fn test_ulid() {
        let a = UlidGenerator::next();
        let b = UlidGenerator::next();
        assert_ne!(a, b);
    }
}
