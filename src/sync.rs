extern crate tic;

#[macro_use]
extern crate clap;

mod common;

use std::sync::mpsc::{channel, Sender};
use std::thread;

use tic::{Clocksource, Sample};

use self::common::*;

fn main() {
    let params = parse_args_or_exit("sync");
    let mut tic_receiver = create_tic_receiver(&params);

    let fr_sender = start_recv_thread(tic_receiver.get_clocksource(), tic_receiver.get_sender());

    for _ in 0..params.sender_threads {
        start_sender_thread(tic_receiver.get_clocksource(), tic_receiver.get_sender(), fr_sender.clone());
    }

    tic_receiver.run();
    print_metrics(&params, tic_receiver.clone_meters());
}

fn start_sender_thread(clock: Clocksource, mut tic_sender: tic::Sender<Metric>, sender: Sender<Timing>) {
    thread::spawn(move || {
        let mut last_iteration = clock.counter();
        loop {
            let now = clock.counter();
            let timing = Timing {
                start: now,
            };
            sender.send(timing).unwrap();
            tic_sender.send(Sample::new(last_iteration, now, Metric::Send)).unwrap();
            last_iteration = now;
        }
    });
}

fn start_recv_thread(clock: Clocksource, mut sender: tic::Sender<Metric>) -> Sender<Timing> {
    let (tx, rx) = channel::<Timing>();
    thread::spawn(move || {
        let mut last_iteration_time = clock.counter();
        loop {
            let timing = rx.recv().expect("failed to receive message");
            let now = clock.counter();
            sender.send(Sample::new(timing.start, now, Metric::RecvLatency)).unwrap();
            sender.send(Sample::new(last_iteration_time, now, Metric::RecvLoopTime)).unwrap();
            last_iteration_time = now;
        }
    });
    tx
}
