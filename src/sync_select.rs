#![feature(mpsc_select)]

extern crate tic;

#[macro_use]
extern crate clap;

mod common;

use std::sync::mpsc::{channel, Sender, Select, Handle};
use std::thread;

use tic::{Clocksource, Sample};

use self::common::*;



fn main() {
    let params = parse_args_or_exit("sync-select");
    let mut tic_receiver = create_tic_receiver(&params);

    let (tx1, tx2) = start_receiver_thread(tic_receiver.get_clocksource(), tic_receiver.get_sender());

    for i in 0..params.sender_threads {
        let sender = if i % 2 == 0 {
            tx1.clone()
        } else {
            tx2.clone()
        };
        start_sender_thread(tic_receiver.get_clocksource(), tic_receiver.get_sender(), sender);
    }

    tic_receiver.run();

    print_metrics(&params, tic_receiver.clone_meters());
}

fn start_receiver_thread(clock: Clocksource, mut sender: tic::Sender<Metric>) -> (Sender<Timing>, Sender<Timing>) {
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();

    thread::spawn(move || {
        loop {
            let start = clock.counter();
            let event: Timing = select!{
                e = rx1.recv() => e.unwrap(),
                e = rx2.recv() => e.unwrap()
            };
            let end = clock.counter();
            sender.send(Sample::new(start, end, Metric::RecvLoopTime)).unwrap();
            sender.send(Sample::new(event.start, end, Metric::RecvLatency)).unwrap();
        }
    });

    (tx1, tx2)
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
