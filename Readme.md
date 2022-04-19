Simple Rust + Actix-Web Service
===
A powerful, pragmatic, and extremely fast web framework for Rust

How to Run
===
* Run `docker-compose up -d` run the run the service with postgress db.
* Or Could be run locally `rustc ./main.rs`

Benchmarks
---

---
Simple Serialization
---

`ab -n 100000 -k -c 30 -q http://localhost:8080/`
```
This is ApacheBench, Version 2.3 <$Revision: 1879490 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient).....done


Server Software:        
Server Hostname:        localhost
Server Port:            8080

Document Path:          /
Document Length:        15 bytes

Concurrency Level:      30
Time taken for tests:   0.841 seconds
Complete requests:      100000
Failed requests:        0
Keep-Alive requests:    100000
Total transferred:      14700000 bytes
HTML transferred:       1500000 bytes
Requests per second:    118839.79 [#/sec] (mean)
Time per request:       0.252 [ms] (mean)
Time per request:       0.008 [ms] (mean, across all concurrent requests)
Transfer rate:          17060.01 [Kbytes/sec] received

Connection Times (ms)
min  mean[+/-sd] median   max
Connect:        0    0   0.0      0       1
Processing:     0    0   0.1      0       3
Waiting:        0    0   0.1      0       3
Total:          0    0   0.1      0       3

Percentage of the requests served within a certain time (ms)
50%      0
66%      0
75%      0
80%      0
90%      0
95%      0
98%      1
99%      1
100%      3 (longest request)

```

DB Read w Docker-Postgres
---

`ab -n 100000 -k -c 30 -q http://localhost:8080/todos/`

```
This is ApacheBench, Version 2.3 <$Revision: 1879490 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient).....done


Server Software:        
Server Hostname:        localhost
Server Port:            8080

Document Path:          /todos/
Document Length:        53 bytes

Concurrency Level:      30
Time taken for tests:   31.720 seconds
Complete requests:      100000
Failed requests:        0
Keep-Alive requests:    100000
Total transferred:      18500000 bytes
HTML transferred:       5300000 bytes
Requests per second:    3152.58 [#/sec] (mean)
Time per request:       9.516 [ms] (mean)
Time per request:       0.317 [ms] (mean, across all concurrent requests)
Transfer rate:          569.56 [Kbytes/sec] received

Connection Times (ms)
min  mean[+/-sd] median   max
Connect:        0    0   0.0      0       1
Processing:     2   10   7.6      8     276
Waiting:        2   10   7.6      8     276
Total:          2   10   7.6      8     276

Percentage of the requests served within a certain time (ms)
50%      8
66%      9
75%     11
80%     12
90%     15
95%     20
98%     27
99%     34
100%    276 (longest request)
```

DB Write e Docker-Postgres
---

`ab -p todo.json -T application/json -n 100000 -k -c 30 -q http://localhost:8080/todos`

```
This is ApacheBench, Version 2.3 <$Revision: 1879490 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient).....done


Server Software:        
Server Hostname:        localhost
Server Port:            8080

Document Path:          /todos
Document Length:        33 bytes

Concurrency Level:      30
Time taken for tests:   38.663 seconds
Complete requests:      100000
Failed requests:        99993
(Connect: 0, Receive: 0, Length: 99993, Exceptions: 0)
Keep-Alive requests:    100000
Total transferred:      16888905 bytes
Total body sent:        19500000
HTML transferred:       3688905 bytes
Requests per second:    2586.48 [#/sec] (mean)
Time per request:       11.599 [ms] (mean)
Time per request:       0.387 [ms] (mean, across all concurrent requests)
Transfer rate:          426.59 [Kbytes/sec] received
492.54 kb/s sent
919.13 kb/s total

Connection Times (ms)
min  mean[+/-sd] median   max
Connect:        0    0   0.0      0       1
Processing:     3   12   6.7     10     140
Waiting:        3   12   6.7     10     140
Total:          3   12   6.7     10     140

Percentage of the requests served within a certain time (ms)
50%     10
66%     12
75%     13
80%     14
90%     19
95%     24
98%     32
99%     38
100%    140 (longest request)

```

