use stack_map::StackMap;

macro_rules! subscribeable_cache {
    ($key_type:ty, $value_type:ty, $buffer_size:expr, $subscriber_size:expr) => {
        pub struct SubscribeableCache {
            cache: StackMap<$key_type, $value_type, $buffer_size>,
            subscribers: StackMap<u64, fn($key_type, Option<$value_type>), $subscriber_size>,
            next_subscriber_id: u64,
        }

        impl SubscribeableCache {
            pub fn new() -> Self {
                Self {
                    cache: StackMap::new(),
                    subscribers: StackMap::new(),
                    next_subscriber_id: 0,
                }
            }

            pub fn subscribe(&mut self, subscriber: fn($key_type, Option<$value_type>)) -> u64 {
                let id = self.next_subscriber_id;
                if self.subscribers.insert(id, subscriber).is_none() {
                    self.next_subscriber_id += 1;
                    id
                } else {
                    u64::MAX // Indicate failure
                }
            }

            pub fn unsubscribe(&mut self, subscription_id: u64) -> bool {
                self.subscribers.remove(&subscription_id).is_some()
            }

            pub fn put(&mut self, key: $key_type, value: $value_type) {
                if self.cache.insert(key, value).is_none() {
                    self.notify_subscribers(key, Some(value));
                }
            }

            pub fn remove(&mut self, key: $key_type) {
                if self.cache.remove(&key).is_some() {
                    self.notify_subscribers(key, None);
                }
            }

            fn notify_subscribers(&self, key: $key_type, value: Option<$value_type>) {
                for (_, subscriber) in self.subscribers.iter() {
                    subscriber(key, value);
                }
            }
        }
    };
}

type Char16 = [char; 16];
type Long = i64;
subscribeable_cache!(Char16, Long, 1024, 128);

fn example_subscriber(key: Char16, value: Option<Long>) {
    match value {
        Some(v) => println!("Subscriber received: {:?} -> {}", key, v),
        None => println!("Subscriber received: {:?} -> NULL", key),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_subscriber(key: Char16, value: Option<Long>) {
        println!("Subscriber notified: {:?} -> {:?}", key, value);
    }

    #[test]
    fn test_initialization() {
        let cache = SubscribeableCache::new();
        assert_eq!(cache.cache.len(), 0);
        assert_eq!(cache.subscribers.len(), 0);
    }

    #[test]
    fn test_subscription() {
        let mut cache = SubscribeableCache::new();
        let id1 = cache.subscribe(test_subscriber);
        let id2 = cache.subscribe(test_subscriber);
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_unsubscription() {
        let mut cache = SubscribeableCache::new();
        let id = cache.subscribe(test_subscriber);
        assert!(cache.unsubscribe(id));
        assert!(!cache.unsubscribe(id)); // Should return false after removal
    }

    #[test]
    fn test_put_and_remove() {
        let mut cache = SubscribeableCache::new();
        let key: Char16 = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'];
        let value: Long = 42;

        cache.put(key, value);
        assert_eq!(cache.cache.get(&key), Some(&value));

        cache.remove(key);
        assert_eq!(cache.cache.get(&key), None);
    }
}
