use disruptor_rs::{
    sequence::Sequence, DisruptorBuilder, EventHandler, EventProcessorExecutor, EventProducer, ExecutorHandle
};

#[derive(Default)]
struct MyEvent {
    value: i32,
}

#[derive(Default)]
struct MyEventHandler;

impl EventHandler<MyEvent> for MyEventHandler {
    fn on_event(&self, event: &MyEvent, sequence: Sequence, _end_of_batch: bool) {
        println!("Event value: {}", event.value);
    }
    fn on_start(&self) {}
    fn on_shutdown(&self) {}
}

#[cfg(test)]
#[test]
fn test_disruptor() {
    let (executor, mut producer) = DisruptorBuilder::with_ring_buffer::<MyEvent>(1024)
    .with_busy_spin_waiting_strategy()
    .with_single_producer_sequencer()
    .with_barrier(|scope| {
        scope.handle_events(MyEventHandler::default());
    })
    .build();

    let handle = executor.spawn();

    let event = MyEvent::default();
    // Write single event
    producer.write(
    std::iter::once(42),
        |event, sequence, &new_value| {
            event.value = new_value;
        }
    );


    producer.drain();
    handle.join();
}

