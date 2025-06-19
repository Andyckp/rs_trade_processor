use ringbuf::*;
use ringbuf::consumer::Consumer;
use ringbuf::producer::Producer;
use stack_map::StackMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Key1(u64);
#[derive(Debug, Clone, Copy)]
struct Value1(u64);
#[derive(Debug, Clone)]
struct Entry1 {
    key: Key1,
    value: Value1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Key2(u64);
#[derive(Debug, Clone, Copy)]
struct Value2(u64);
#[derive(Debug, Clone)]
struct Entry2 {
    key: Key2,
    value: Value2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Key3(u64);
#[derive(Debug, Clone, Copy)]
struct Value3(u64);
#[derive(Debug, Clone)]
struct Entry3 {
    key: Key3,
    value: Value3,
}

macro_rules! subscribeable_cache2 {
    ($name:ident, $key_type:ty, $value_type:ty, $buffer_size:expr, $subscriber_size:expr) => {
        pub struct $name {
            pub cache: StackMap<$key_type, $value_type, $buffer_size>,
            pub subscribers: [(Option<fn($key_type, Option<$value_type>)>); $subscriber_size],
            pub next_subscriber_id: usize,
        }

        impl $name {
            pub fn subscribe(&mut self, subscriber: fn($key_type, Option<$value_type>)) -> Option<usize> {
                for i in 0..$subscriber_size {
                    if self.subscribers[i].is_none() {
                        self.subscribers[i] = Some(subscriber);
                        self.next_subscriber_id = i + 1;
                        return Some(i);
                    }
                }
                None
            }

            pub fn unsubscribe(&mut self, id: usize) -> bool {
                if id < $subscriber_size {
                    self.subscribers[id] = None;
                    true
                } else {
                    false
                }
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
                let mut i = 0;
                while i < $subscriber_size {
                    if let Some(sub) = self.subscribers[i] {
                        sub(key, value);
                    }
                    i += 1;
                }
            }
        }
    };
}

subscribeable_cache2!(Cache1, Key1, Value1, 64, 8);
subscribeable_cache2!(Cache2, Key2, Value2, 64, 8);
subscribeable_cache2!(Cache3, Key3, Value3, 64, 8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        fn subscriber1(key: Key1, value: Option<Value1>) {
            println!("[Subscriber1] Key: {:?}, Value: {:?}", key, value);
        }

        fn subscriber2(key: Key2, value: Option<Value2>) {
            println!("[Subscriber2] Key: {:?}, Value: {:?}", key, value);
        }

        fn subscriber3(key: Key3, value: Option<Value3>) {
            println!("[Subscriber3] Key: {:?}, Value: {:?}", key, value);
        }

        const NONE_FN1: Option<fn(Key1, Option<Value1>)> = None;
        const NONE_FN2: Option<fn(Key2, Option<Value2>)> = None;
        const NONE_FN3: Option<fn(Key3, Option<Value3>)> = None;

        let mut cache1 = Cache1 {
            cache: StackMap::new(),
            subscribers: [NONE_FN1; 8],
            next_subscriber_id: 0,
        };

        let mut cache2 = Cache2 {
            cache: StackMap::new(),
            subscribers: [NONE_FN2; 8],
            next_subscriber_id: 0,
        };

        let mut cache3 = Cache3 {
            cache: StackMap::new(),
            subscribers: [NONE_FN3; 8],
            next_subscriber_id: 0,
        };

        cache1.subscribe(subscriber1);
        cache2.subscribe(subscriber2);
        cache3.subscribe(subscriber3);

        cache1.put(Key1(1), Value1(101));
        cache2.put(Key2(2), Value2(202));
        cache3.put(Key3(3), Value3(303));

        cache1.remove(Key1(1));
        cache2.remove(Key2(2));
        cache3.remove(Key3(3));
    }
}
