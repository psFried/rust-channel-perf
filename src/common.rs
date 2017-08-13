use tic::{self, Interest, Percentile};

#[derive(Debug, PartialEq, Clone)]
pub struct RunParameters {
    pub name: &'static str,
    pub sender_threads: usize,
    pub windows: usize,
    pub secs_per_window: usize,
}

pub fn parse_args_or_exit(exe: &'static str) -> RunParameters {
    use clap::{App, Arg};

    let app = App::new(exe)
            .arg(Arg::with_name("sender-threads")
                    .short("s")
                    .long("sender-threads")
                    .default_value("2"))
            .arg(Arg::with_name("windows")
                    .short("w")
                    .long("windows")
                    .default_value("4"))
            .arg(Arg::with_name("secs-per-window")
                    .short("t")
                    .long("secs-per-window")
                    .default_value("5"));
    let matches = app.get_matches();

    let senders = value_t_or_exit!(matches, "sender-threads", usize);
    let windows = value_t_or_exit!(matches, "windows", usize);
    let secs = value_t_or_exit!(matches, "secs-per-window", usize);


    let params = RunParameters {
        name: exe,
        sender_threads: senders,
        windows: windows,
        secs_per_window: secs
    };
    println!("Running with parameters: {:?}", params);
    params
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Metric {
    RecvLatency,
    RecvLoopTime,
    Send,
}

impl Metric {
    pub fn all() -> Vec<Metric> {
        use self::Metric::*;
        vec![RecvLatency, RecvLoopTime, Send]
    }
}

impl ::std::fmt::Display for Metric {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Timing {
    pub start: u64,
}

pub fn create_tic_receiver(params: &RunParameters) -> tic::Receiver<Metric> {
    let mut recv = tic::Receiver::configure()
            .windows(params.windows)
            .duration(params.secs_per_window)
            .batch_size(512)
            .build();

    for metric in Metric::all() {
        recv.add_interest(Interest::Count(metric));
        recv.add_interest(Interest::Percentile(metric));
    }

    recv
}

pub fn print_metrics(params: &RunParameters, meters: tic::Meters<Metric>) {
    println!("Finished: {}", params.name);

    for metric in Metric::all() {
        let count = meters.count(&metric).expect("failed to get count");
        let rate = (*count as f64) / (params.windows * params.secs_per_window) as f64;

        println!("{}:", metric);
        println!("\tTotal count: {}, rate (per second): {}", count, rate);
        println!("\tlatency (ns): p50: {} p90: {} p9999: {}",
                 meters.percentile(&metric, Percentile("p50".to_owned(), 50.0))
                        .unwrap_or(&0),
                 meters.percentile(&metric, Percentile("p90".to_owned(), 90.0))
                        .unwrap_or(&0),
                 meters.percentile(&metric, Percentile("p9999".to_owned(), 99.99))
                        .unwrap_or(&0)
        );
    }
}
