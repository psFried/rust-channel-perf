extern crate tokio_core;
extern crate futures;
extern crate tic;

#[macro_use]
extern crate clap;

mod common;

use std::thread;

use tic::Sample;
use futures::sync::mpsc::{channel, Sender};
use futures::Stream;
use tokio_core::reactor::Core;

use self::common::*;

fn main() {
    let params = parse_args_or_exit("async");
    let mut tic_receiver = create_tic_receiver(&params);
    let fr_sender = start_recv_thread(tic_receiver.get_clocksource(), tic_receiver.get_sender());

    for _ in 0..params.sender_threads {
        start_sender_thread(tic_receiver.get_clocksource(), tic_receiver.get_sender(), fr_sender.clone());
    }

    tic_receiver.run();
    print_metrics(&params, tic_receiver.clone_meters());
}

fn start_sender_thread(clock: tic::Clocksource, mut tic_sender: tic::Sender<Metric>, sender: Sender<Timing>) {
    thread::spawn(move || {
        let mut reactor = Core::new().unwrap();

        let mut last_iteration_time = clock.counter();
        let stream = ::futures::stream::iter(StartIterator{
            clock: clock.clone()
        }).map(|timing| {
            let now = clock.counter();
            tic_sender.send(Sample::new(last_iteration_time, now, Metric::Send)).unwrap();
            last_iteration_time = now;
            timing
        });
        let future = stream.forward(sender);
        reactor.run(future).unwrap();
    });
}


fn start_recv_thread(clock: tic::Clocksource, mut tic_sender: tic::Sender<Metric>) -> Sender<Timing> {
    let (tx, rx) = channel(500000);
    thread::spawn(move || {
        let mut reactor = Core::new().unwrap();       

        let mut last_iteration = clock.counter();

        let future = rx.for_each(|timing: Timing| {
            let now = clock.counter();
            let sample = tic::Sample::new(timing.start, now, Metric::RecvLatency);
            tic_sender.send(sample).unwrap();
            let sample = tic::Sample::new(last_iteration, now, Metric::RecvLoopTime);
            tic_sender.send(sample).unwrap();
            last_iteration = now;

            Ok(())
        });

        println!("Starting recv receiver");
        let _ = reactor.run(future);
    });
    tx
}

pub struct StartIterator {
    clock: tic::Clocksource,
}

impl Iterator for StartIterator {
    type Item = Result<Timing, MyErr>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Ok(Timing{
            start: self.clock.counter(),
        }))
    }
}

#[derive(Debug)]
pub struct MyErr;

impl <T> From<::futures::sync::mpsc::SendError<T>> for MyErr {
    fn from(_: ::futures::sync::mpsc::SendError<T>) -> MyErr {
        MyErr
    }
}

impl From<&'static str> for MyErr {
    fn from(_: &'static str) -> MyErr {
        MyErr
    }
}
