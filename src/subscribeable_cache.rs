use std::{sync::Arc, thread};

use ringbuf::{traits::{Producer, SplitRef}, StaticRb};
use ringbuf::{traits::*};
use stack_map::StackMap;
use std::collections::HashMap;

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

macro_rules! subscribeable_cache {
    ($name:ident, $key_type:ty, $value_type:ty, $buffer_size:expr, $subscriber_size:expr) => {
        pub struct $name {
            cache: StackMap<$key_type, $value_type, $buffer_size>,
            subscribers: HashMap<u64, Box<dyn FnMut($key_type, Option<$value_type>) + Send>>,
            next_subscriber_id: u64,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    cache: StackMap::new(),
                    subscribers: HashMap::new(),
                    next_subscriber_id: 0,
                }
            }

            pub fn subscribe(&mut self, mut subscriber: Box<dyn FnMut($key_type, Option<$value_type>) + Send>) -> u64 {
                let id = self.next_subscriber_id;

                // Temporarily invoke the subscriber before moving it into the map
                for (key, value) in self.cache.iter() {
                    subscriber(*key, Some(*value));
                }

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

            fn notify_subscribers(&mut self, key: $key_type, value: Option<$value_type>) {
                for (_, subscriber) in self.subscribers.iter_mut() {
                    subscriber(key, value);
                }
            }
        }
    };
}


subscribeable_cache!(Cache1, Key1, Value1, 256, 8);
subscribeable_cache!(Cache2, Key2, Value2, 256, 8);
subscribeable_cache!(Cache3, Key3, Value3, 256, 8);

// static mut CACHE_1: Cache1 = Cache1::new();
// static mut CACHE_2: Cache2 = Cache2::new();
// static mut CACHE_3: Cache3 = Cache3::new();


pub struct CompositeCache {
    rb1: StaticRb<Entry1, 64>,
    rb2: StaticRb<Entry2, 64>,
    rb3: StaticRb<Entry3, 64>,

    cache1: Cache1,
    cache2: Cache2,
    cache3: Cache3,
}

// impl CompositeCache {
//     pub fn new() -> Self {
//         Self {
//             rb1: StaticRb::default(),
//             rb2: StaticRb::default(),
//             rb3: StaticRb::default(),
//             cache1: Cache1::new(),
//             cache2: Cache2::new(),
//             cache3: Cache3::new(),
//         }
//     }

//     pub fn run(&mut self) {
//         let (_, mut cons1) = self.rb1.split_ref();
//         let (_, mut cons2) = self.rb2.split_ref();
//         let (_, mut cons3) = self.rb3.split_ref();

//         loop {
//             while let Some(entry) = cons1.try_pop() {
//                 self.cache1.put(entry.key, entry.value);
//             }
//             while let Some(entry) = cons2.try_pop() {
//                 self.cache2.put(entry.key, entry.value);
//             }
//             while let Some(entry) = cons3.try_pop() {
//                 self.cache3.put(entry.key, entry.value);
//             }

//             // Optional: pacing
//         }
//     }

//     pub fn put1(&mut self, key: Key1, value: Value1) -> bool {
//         let (mut prod, _) = self.rb1.split_ref();
//         prod.try_push(Entry1 { key, value }).is_ok()
//     }

//     pub fn put2(&mut self, key: Key2, value: Value2) -> bool {
//         let (mut prod, _) = self.rb2.split_ref();
//         prod.try_push(Entry2 { key, value }).is_ok()
//     }

//     pub fn put3(&mut self, key: Key3, value: Value3) -> bool {
//         let (mut prod, _) = self.rb3.split_ref();
//         prod.try_push(Entry3 { key, value }).is_ok()
//     }

//     pub fn subscribe1(&mut self, f: fn(Key1, Option<Value1>)) -> u64 {
//         self.cache1.subscribe(f)
//     }

//     pub fn unsubscribe1(&mut self, id: u64) -> bool {
//         self.cache1.unsubscribe(id)
//     }

//     pub fn subscribe2(&mut self, f: fn(Key2, Option<Value2>)) -> u64 {
//         self.cache2.subscribe(f)
//     }

//     pub fn unsubscribe2(&mut self, id: u64) -> bool {
//         self.cache2.unsubscribe(id)
//     }

//     pub fn subscribe3(&mut self, f: fn(Key3, Option<Value3>)) -> u64 {
//         self.cache3.subscribe(f)
//     }

//     pub fn unsubscribe3(&mut self, id: u64) -> bool {
//         self.cache3.unsubscribe(id)
//     }

//     pub fn start(mut self) {
//         thread::spawn(move || {
//             self.run();
//         });
//     }
// }

#[test]
fn test_three_producers_and_one_consumer() {
    let rb1 = StaticRb::<Entry1, 256>::default();
    let rb2 = StaticRb::<Entry2, 256>::default();
    let rb3 = StaticRb::<Entry3, 256>::default();

    let (mut prod1, mut cons1) = rb1.split();
    let (mut prod2, mut cons2) = rb2.split();
    let (mut prod3, mut cons3) = rb3.split();

    // Producer 1
    let t1 = thread::spawn(move || {
        for i in 0..100 {
            while prod1.try_push(Entry1 { key: Key1(i), value: Value1(i * 10) }).is_err() {
                thread::yield_now();
            }
        }
    });

    // Producer 2
    let t2 = thread::spawn(move || {
        for i in 0..100 {
            while prod2.try_push(Entry2 { key: Key2(i), value: Value2(i * 10) }).is_err() {
                thread::yield_now();
            }
        }
    });

    // Producer 3
    let t3 = thread::spawn(move || {
        for i in 0..100 {
            while prod3.try_push(Entry3 { key: Key3(i), value: Value3(i * 10) }).is_err() {
                thread::yield_now();
            }
        }
    });

    // Consumer thread
    let consumer = thread::spawn(move || {
        let mut c1 = Cache1::new();
        let mut c2: Cache2 = Cache2::new();
        let mut c3 = Cache3::new();

        // c1.subscribe(|k, v| println!("Cache1: key={:?}, val={:?}", k.0, v.map(|v| v.0)));
        // c2.subscribe(|k, v| println!("Cache2: key={:?}, val={:?}", k.0, v.map(|v| v.0)));
        // c3.subscribe(|k, v| println!("Cache3: key={:?}, val={:?}", k.0, v.map(|v| v.0)));

        c1.subscribe(Box::new(|k, v| println!("Cache1: key={:?}, val={:?}", k.0, v.map(|v| v.0))));
        c2.subscribe(Box::new(|k, v| println!("Cache2: key={:?}, val={:?}", k.0, v.map(|v| v.0))));
        c3.subscribe(Box::new(|k, v| println!("Cache3: key={:?}, val={:?}", k.0, v.map(|v| v.0))));


        while c1.cache.len() < 100 || c2.cache.len() < 100 || c3.cache.len() < 100 {
            if let Some(e) = cons1.try_pop() {
                c1.put(e.key, e.value);
            }
            if let Some(e) = cons2.try_pop() {
                c2.put(e.key, e.value);
            }
            if let Some(e) = cons3.try_pop() {
                c3.put(e.key, e.value);
            }
        }

        (c1, c2, c3)
    });

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();

    let (final_c1, final_c2, final_c3) = consumer.join().unwrap();
    assert_eq!(final_c1.cache.len(), 100);
    assert_eq!(final_c2.cache.len(), 100);
    assert_eq!(final_c3.cache.len(), 100);
}


#[test]
fn test_independent_cache() {
    let rb_data_1 = StaticRb::<Entry1, 256>::default();
    // let rb_sub_1 = StaticRb::<fn(Key1, Option<Value1>), 256>::default();
    let rb_sub_1 = StaticRb::<Box<dyn FnMut(Key1, Option<Value1>) + Send>, 256>::default();

    let (mut prod_data_1, mut cons1) = rb_data_1.split();
    let (mut sub_prod_1, mut sub_cons1) = rb_sub_1.split();
 
    let mut rb_downstream = StaticRb::<Entry1, 256>::default();
    let (mut prod_downstream, mut cons_downstream) = rb_downstream.split();

    // Producer 1
    let t1 = thread::spawn(move || {
        for i in 0..100 {
            while prod_data_1.try_push(Entry1 { key: Key1(i), value: Value1(i * 10) }).is_err() {
                thread::yield_now();
            }
        }
    });    
    
    // Consumer thread
    let consumer = thread::spawn(move || {
        let mut c1 = Cache1::new();
        while c1.cache.len() < 100 || c1.subscribers.len() < 1 {
            if let Some(e) = sub_cons1.try_pop() {
                c1.subscribe(e);
            }

            if let Some(e) = cons1.try_pop() {
                c1.put(e.key, e.value);
            }
         }
        (c1)
    });

    // sleep for a moment to ensure consumer is ready
    thread::sleep(std::time::Duration::from_millis(1));

    // sub_prod_1.try_push(|k, v| {
    //     println!("Subscriber: key={:?}, val={:?}", k.0, v.map(|v| v.0));
    //     prod_downstream.try_push(Entry1 { key: k, value: v.unwrap_or(Value1(0)) }).unwrap();
    // });

    // sub_prod_1.try_push(Box::new(move |k, v| {
    //     println!("Subscriber: key={:?}, val={:?}", k.0, v.map(|v| v.0));
    //     prod_downstream
    //         .try_push(Entry1 { key: k, value: v.unwrap_or(Value1(0)) })
    //         .unwrap();
    // }));

    // let mut prod_downstream = ...; // needs to be mutable

    // let elem = Box::new(move |k: Key1, v: Option<Value1>| {
    //     println!("Subscriber: key={:?}, val={:?}", k.0, v.map(|v| v.0));
    //     prod_downstream.try_push(Entry1 { key: k, value: v.unwrap_or(Value1(0)) }).unwrap();
    // });
    sub_prod_1.try_push(Box::new(move |k: Key1, v: Option<Value1>| {
        println!("Subscriber: key={:?}, val={:?}", k.0, v.map(|v| v.0));
        prod_downstream.try_push(Entry1 { key: k, value: v.unwrap_or(Value1(0)) }).unwrap();
    }));


    thread::sleep(std::time::Duration::from_millis(1));
    
    t1.join().unwrap();
    let (final_c1) = consumer.join().unwrap();
    assert_eq!(final_c1.cache.len(), 100);
 }

