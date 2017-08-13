# Rust Channel Performance Tests

I created this project so that I could start to understand the performance characteristics of different types of channels in rust. The aim is to get a general idea of the performance of each type of channel. This isn't meant to be a rigorous benchmark.

### This project builds a separate binary for each performance test:

- `sync` uses the builtin `std::sync::mpsc::{Sender, Receiver}` to send and receive messages.
- `async` uses the tokio and futures crates, and channels from `futures::sync::mpsc`

So far, it seems that the channels in the `sync` tests are about 12 times faster (at least on my machine). There does not seem to be much difference performance wise between the bounded and unbounded variants in the futures crate. 


## Results from my machine (2014 macbook pro, 2.8 gHz i7, 16GB)

### Using `std::sync::mpsc`

```
Running with parameters: RunParameters { name: "sync", sender_threads: 1, windows: 4, secs_per_window: 5 }
Finished: sync
RecvLatency:
	Total count: 32532736, rate (per second): 1626636.8
	latency (ns): p50: 8435315770 p90: 9311489098 p9999: 9594956940
RecvLoopTime:
	Total count: 32532736, rate (per second): 1626636.8
	latency (ns): p50: 88 p90: 95 p9999: 452199
Send:
	Total count: 61025792, rate (per second): 3051289.6
	latency (ns): p50: 71 p90: 77 p9999: 392168
```

Sending is ridiculously fast using the standard channels, so fast that a single sender can outpace the receiver. So it's no surprise that the latency for receiving each message is really high, since the channel must be developing quite the backlog.

### Using `futures::sync::mpsc`

With 2 senders:

```
Running with parameters: RunParameters { name: "async", sender_threads: 2, windows: 4, secs_per_window: 5 }
Starting recv receiver
Finished: async
RecvLatency:
	Total count: 4033792, rate (per second): 201689.6
	latency (ns): p50: 6034 p90: 7148 p9999: 4324328
RecvLoopTime:
	Total count: 4033792, rate (per second): 201689.6
	latency (ns): p50: 2296 p90: 10101 p9999: 95224
Send:
	Total count: 4033536, rate (per second): 201676.8
	latency (ns): p50: 9700 p90: 12796 p9999: 166200
```

With 4 senders:

```
Running with parameters: RunParameters { name: "async", sender_threads: 4, windows: 4, secs_per_window: 5 }
Starting recv receiver
Finished: async
RecvLatency:
	Total count: 5003264, rate (per second): 250163.2
	latency (ns): p50: 9167 p90: 12805 p9999: 558892
RecvLoopTime:
	Total count: 5003264, rate (per second): 250163.2
	latency (ns): p50: 156 p90: 15418 p9999: 33620
Send:
	Total count: 5002752, rate (per second): 250137.6
	latency (ns): p50: 15770 p90: 16581 p9999: 42959
```

With 8 senders:

```
Running with parameters: RunParameters { name: "async", sender_threads: 8, windows: 4, secs_per_window: 5 }
Starting recv receiver
Finished: async
RecvLatency:
	Total count: 5528576, rate (per second): 276428.8
	latency (ns): p50: 15729 p90: 25035 p9999: 7813989
RecvLoopTime:
	Total count: 5528576, rate (per second): 276428.8
	latency (ns): p50: 154 p90: 27050 p9999: 51348
Send:
	Total count: 5525504, rate (per second): 276275.2
	latency (ns): p50: 28705 p90: 29967 p9999: 77726
```
