# Rust Channel Performance Tests

I created this project so that I could start to understand the performance characteristics of different types of channels in rust. The aim is to get a general idea of the performance of each type of channel. This isn't meant to be a rigorous benchmark.

### This project builds a separate binary for each performance test:

- `sync` uses the builtin `std::sync::mpsc::{Sender, Receiver}` to send and receive messages.
- `async` uses the tokio and futures crates, and channels from `futures::sync::mpsc`

So far, it seems that the channels in the `sync` tests are about 12 times faster (at least on my machine). There does not seem to be much difference performance wise between the bounded and unbounded variants in the futures crate. 


## Results from my machine (2014 macbook pro, 2.8 gHz i7, 16GB)

### Sync: 

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

### Async:

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

## Sync using select! macro

An important feature is the ability to receive a message from one of multiple possible receivers. For the channels in std, this requires a nightly compiler and enabling the `mpsc_select` feature.

```
Running with parameters: RunParameters { name: "sync-select", sender_threads: 2, windows: 4, secs_per_window: 5 }
Finished: sync-select
RecvLatency:
	Total count: 20180992, rate (per second): 1009049.6
	latency (ns): p50: 8924942042 p90: 9990093931 p9999: 10230612100
RecvLoopTime:
	Total count: 20180992, rate (per second): 1009049.6
	latency (ns): p50: 92 p90: 100 p9999: 2714
Send:
	Total count: 78127104, rate (per second): 3906355.2
	latency (ns): p50: 82 p90: 91 p9999: 660603
```

## Async selecting from 2 receivers

//TODO

## Repeating the tests on an Amazon EC2 m4.4xlarge instance 

### Async:

```
Running with parameters: RunParameters { name: "async", sender_threads: 1, windows: 4, secs_per_window: 5 }
Starting recv receiver
Finished: async
RecvLatency:
	Total count: 25995776, rate (per second): 1299788.8
	latency (ns): p50: 1548 p90: 11273 p9999: 23987
RecvLoopTime:
	Total count: 25995776, rate (per second): 1299788.8
	latency (ns): p50: 453 p90: 1584 p9999: 22938
Send:
	Total count: 25995776, rate (per second): 1299788.8
	latency (ns): p50: 703 p90: 1090 p9999: 14435
	
Running with parameters: RunParameters { name: "async", sender_threads: 2, windows: 4, secs_per_window: 5 }
Starting recv receiver
Finished: async
RecvLatency:
	Total count: 45148416, rate (per second): 2257420.8
	latency (ns): p50: 9865004 p90: 175019918 p9999: 198239585
RecvLoopTime:
	Total count: 45148416, rate (per second): 2257420.8
	latency (ns): p50: 394 p90: 578 p9999: 10986
Send:
	Total count: 45619712, rate (per second): 2280985.6
	latency (ns): p50: 798 p90: 1054 p9999: 12370
	
Running with parameters: RunParameters { name: "async", sender_threads: 3, windows: 4, secs_per_window: 5 }
Starting recv receiver
Finished: async
RecvLatency:
	Total count: 31100416, rate (per second): 1555020.8
	latency (ns): p50: 340913030 p90: 355408544 p9999: 363998479
RecvLoopTime:
	Total count: 31100416, rate (per second): 1555020.8
	latency (ns): p50: 430 p90: 1025 p9999: 18744
Send:
	Total count: 31600128, rate (per second): 1580006.4
	latency (ns): p50: 1016 p90: 2816 p9999: 37225
	
Running with parameters: RunParameters { name: "async", sender_threads: 4, windows: 4, secs_per_window: 5 }
Starting recv receiver
Finished: async
RecvLatency:
	Total count: 21484544, rate (per second): 1074227.2
	latency (ns): p50: 470030484 p90: 641023869 p9999: 716185797
RecvLoopTime:
	Total count: 21484544, rate (per second): 1074227.2
	latency (ns): p50: 450 p90: 2386 p9999: 19891
Send:
	Total count: 21983744, rate (per second): 1099187.2
	latency (ns): p50: 1218 p90: 14803 p9999: 44499
```

### Sync:

```
Running with parameters: RunParameters { name: "sync", sender_threads: 1, windows: 4, secs_per_window: 5 }
Finished: sync
RecvLatency:
	Total count: 62380800, rate (per second): 3119040
	latency (ns): p50: 59147 p90: 9026143 p9999: 9445573
RecvLoopTime:
	Total count: 62380800, rate (per second): 3119040
	latency (ns): p50: 284 p90: 379 p9999: 9888
Send:
	Total count: 62410240, rate (per second): 3120512
	latency (ns): p50: 302 p90: 420 p9999: 10576
	
Running with parameters: RunParameters { name: "sync", sender_threads: 2, windows: 4, secs_per_window: 5 }
Finished: sync
RecvLatency:
	Total count: 51998720, rate (per second): 2599936
	latency (ns): p50: 6373731468 p90: 6966436955 p9999: 7116760810
RecvLoopTime:
	Total count: 51998720, rate (per second): 2599936
	latency (ns): p50: 340 p90: 460 p9999: 11101
Send:
	Total count: 81327616, rate (per second): 4066380.8
	latency (ns): p50: 462 p90: 590 p9999: 13738
	
Running with parameters: RunParameters { name: "sync", sender_threads: 3, windows: 4, secs_per_window: 5 }
Finished: sync
RecvLatency:
    Total count: 43709952, rate (per second): 2185497.6
    latency (ns): p50: 9801115370 p90: 11012296147 p9999: 11330123727
RecvLoopTime:
    Total count: 43709952, rate (per second): 2185497.6
    latency (ns): p50: 430 p90: 565 p9999: 12026
Send:
    Total count: 100680192, rate (per second): 5034009.6
    latency (ns): p50: 560 p90: 715 p9999: 13239
    
Running with parameters: RunParameters { name: "sync", sender_threads: 4, windows: 4, secs_per_window: 5 }
Finished: sync
RecvLatency:
    Total count: 37764608, rate (per second): 1888230.4
    latency (ns): p50: 11622181503 p90: 12893491823 p9999: 13237089207
RecvLoopTime:
    Total count: 37764608, rate (per second): 1888230.4
    latency (ns): p50: 494 p90: 649 p9999: 12706
Send:
    Total count: 111678976, rate (per second): 5583948.8
    latency (ns): p50: 689 p90: 859 p9999: 14525
```

