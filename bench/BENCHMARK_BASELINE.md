# Django-Bolt Benchmark
Generated: Sun 22 Mar 2026 01:17:17 PM PKT
Config: 8 processes × 1 workers | C=100 N=10000

## Root Endpoint Performance
  Reqs/sec    127779.99   13504.25  139496.35
  Latency      763.07us   589.88us     6.08ms
  Latency Distribution
     50%   592.00us
     75%     0.85ms
     90%     1.40ms
     99%     3.71ms

## 10kb JSON Response Performance
### 10kb JSON (Async) (/10k-json)
  Reqs/sec     88560.05    8710.09  102193.51
  Latency        1.08ms   714.27us     8.06ms
  Latency Distribution
     50%   845.00us
     75%     1.31ms
     90%     2.04ms
     99%     4.43ms
### 10kb JSON (Sync) (/sync-10k-json)
  Reqs/sec     99728.02   16412.66  128009.63
  Latency        1.03ms   545.50us     6.60ms
  Latency Distribution
     50%     0.90ms
     75%     1.26ms
     90%     1.82ms
     99%     3.71ms

## Response Type Endpoints
### Header Endpoint (/header)
  Reqs/sec     78150.36    7791.16   88230.38
  Latency        1.28ms   594.07us     7.54ms
  Latency Distribution
     50%     1.11ms
     75%     1.59ms
     90%     2.24ms
     99%     3.78ms
### Cookie Endpoint (/cookie)
  Reqs/sec     79703.51    7689.84   87535.15
  Latency        1.22ms   470.02us     5.60ms
  Latency Distribution
     50%     1.12ms
     75%     1.55ms
     90%     2.04ms
     99%     3.27ms
### Exception Endpoint (/exc)
  Reqs/sec    105827.40   17976.61  123180.62
  Latency        0.93ms   561.87us     7.43ms
  Latency Distribution
     50%   753.00us
     75%     1.18ms
     90%     1.70ms
     99%     3.55ms
### HTML Response (/html)
  Reqs/sec    130783.66   10215.22  141821.97
  Latency      750.67us   454.13us     5.04ms
  Latency Distribution
     50%   598.00us
     75%     0.90ms
     90%     1.32ms
     99%     3.32ms
### Redirect Response (/redirect)
### File Static via FileResponse (/file-static)
  Reqs/sec     29915.17    7135.29   36031.73
  Latency        3.32ms     1.67ms    19.31ms
  Latency Distribution
     50%     2.88ms
     75%     3.87ms
     90%     5.46ms
     99%    10.33ms

## Authentication & Authorization Performance
### Auth NO User Access (/auth/no-user-access) - lazy loading, no DB query
  Reqs/sec     63803.82    6377.83   70525.04
  Latency        1.53ms   620.08us     6.25ms
  Latency Distribution
     50%     1.35ms
     75%     1.89ms
     90%     2.56ms
     99%     4.22ms
### Get Authenticated User (/auth/me) - accesses request.user, triggers DB query
  Reqs/sec     16339.78    1868.63   18668.63
  Latency        6.09ms     1.72ms    19.12ms
  Latency Distribution
     50%     5.91ms
     75%     7.27ms
     90%     8.52ms
     99%    12.35ms
### Get User via Dependency (/auth/me-dependency)
 8775 / 10000 [============================================>------]  87.75% 14590/s
  Reqs/sec     14727.17    1360.47   17411.82
  Latency        6.78ms     2.58ms    22.56ms
  Latency Distribution
     50%     6.22ms
     75%     8.29ms
     90%    10.91ms
     99%    15.20ms
### Get Auth Context (/auth/context) validated jwt no db
  Reqs/sec     71394.87    7101.08   78979.16
  Latency        1.38ms   539.46us     5.63ms
  Latency Distribution
     50%     1.27ms
     75%     1.70ms
     90%     2.22ms
     99%     3.87ms

## Items GET Performance (/items/1?q=hello)
  Reqs/sec    116696.75   13340.89  132900.71
  Latency        0.86ms   566.27us     5.76ms
  Latency Distribution
     50%   694.00us
     75%     0.97ms
     90%     1.54ms
     99%     3.88ms

## Items PUT JSON Performance (/items/1)
  Reqs/sec     99293.40   14894.81  123629.34
  Latency        0.96ms   708.93us     8.20ms
  Latency Distribution
     50%   733.00us
     75%     1.09ms
     90%     1.69ms
     99%     4.38ms

## ORM Performance
Seeding 1000 users for benchmark...
Successfully seeded users
Validated: 10 users exist in database
### Users Full10 (Async) (/users/full10)
  Reqs/sec     14460.83    1560.66   16952.32
  Latency        6.90ms     2.83ms    24.42ms
  Latency Distribution
     50%     6.28ms
     75%     8.38ms
     90%    11.15ms
     99%    17.12ms
### Users Full10 (Sync) (/users/sync-full10)
 2390 / 10000 [============>--------------------------------------]  23.90% 11876/s
 7475 / 10000 [======================================>------------]  74.75% 12409/s
  Reqs/sec     12682.37    2156.43   17445.43
  Latency        7.87ms     3.05ms    34.10ms
  Latency Distribution
     50%     7.12ms
     75%     9.37ms
     90%    12.40ms
     99%    18.48ms
### Users Mini10 (Async) (/users/mini10)
 7175 / 10000 [====================================>--------------]  71.75% 17903/s
  Reqs/sec     17297.54    2946.85   19830.42
  Latency        5.61ms     2.16ms    22.77ms
  Latency Distribution
     50%     5.13ms
     75%     6.88ms
     90%     8.72ms
     99%    13.33ms
### Users Mini10 (Sync) (/users/sync-mini10)
  Reqs/sec     14050.51    2052.04   23157.13
  Latency        7.22ms     2.43ms    31.00ms
  Latency Distribution
     50%     6.74ms
     75%     8.68ms
     90%    10.86ms
     99%    15.59ms
Cleaning up test users...

## Class-Based Views (CBV) Performance
### Simple APIView GET (/cbv-simple)
  Reqs/sec     85352.48    7617.83   94557.23
  Latency        1.14ms   501.76us     6.29ms
  Latency Distribution
     50%     1.02ms
     75%     1.43ms
     90%     1.93ms
     99%     3.35ms
### Simple APIView POST (/cbv-simple)
  Reqs/sec     84708.75    7902.30   95159.60
  Latency        1.15ms   481.61us     5.81ms
  Latency Distribution
     50%     1.04ms
     75%     1.42ms
     90%     1.85ms
     99%     3.46ms
### Items100 ViewSet GET (/cbv-items100)
  Reqs/sec     59956.02    5914.45   65916.38
  Latency        1.66ms   658.16us     9.89ms
  Latency Distribution
     50%     1.50ms
     75%     1.96ms
     90%     2.61ms
     99%     4.21ms

## CBV Items - Basic Operations
### CBV Items GET (Retrieve) (/cbv-items/1)
  Reqs/sec     87450.18   10907.96   98454.15
  Latency        1.13ms   463.99us     6.07ms
  Latency Distribution
     50%     1.02ms
     75%     1.39ms
     90%     1.82ms
     99%     3.25ms
### CBV Items PUT (Update) (/cbv-items/1)
  Reqs/sec     84571.14    6659.40   92039.85
  Latency        1.16ms   499.43us     7.13ms
  Latency Distribution
     50%     1.04ms
     75%     1.44ms
     90%     1.93ms
     99%     3.48ms

## CBV Additional Benchmarks
### CBV Bench Parse (POST /cbv-bench-parse)
  Reqs/sec     84350.54   10108.92   92438.17
  Latency        1.17ms   500.04us     5.33ms
  Latency Distribution
     50%     1.04ms
     75%     1.47ms
     90%     1.99ms
     99%     3.65ms
### CBV Response Types (/cbv-response)
  Reqs/sec     87352.87   10124.24   97754.18
  Latency        1.10ms   553.04us     8.52ms
  Latency Distribution
     50%     0.97ms
     75%     1.35ms
     90%     1.76ms
     99%     3.27ms

## ORM Performance with CBV
Seeding 1000 users for CBV benchmark...
Successfully seeded users
Validated: 10 users exist in database
### Users CBV Mini10 (List) (/users/cbv-mini10)
  Reqs/sec     15337.62    2342.91   17892.17
  Latency        6.40ms     2.42ms    20.59ms
  Latency Distribution
     50%     5.94ms
     75%     7.79ms
     90%    10.10ms
     99%    14.36ms
Cleaning up test users...


## Form and File Upload Performance
### Form Data (POST /form)
  Reqs/sec     99174.44   15755.51  114068.16
  Latency        1.00ms   618.22us     6.45ms
  Latency Distribution
     50%   783.00us
     75%     1.24ms
     90%     1.91ms
     99%     4.24ms
### File Upload (POST /upload)
  Reqs/sec     94645.76   11463.90  114118.10
  Latency        1.06ms   613.23us     5.99ms
  Latency Distribution
     50%     0.90ms
     75%     1.25ms
     90%     1.78ms
     99%     4.10ms
### Mixed Form with Files (POST /mixed-form)
  Reqs/sec     91297.79    8592.64   99264.11
  Latency        1.08ms   543.33us     6.97ms
  Latency Distribution
     50%     0.95ms
     75%     1.36ms
     90%     1.82ms
     99%     3.69ms

## Django Middleware Performance
### Django Middleware + Messages Framework (/middleware/demo)
Tests: SessionMiddleware, AuthenticationMiddleware, MessageMiddleware, custom middleware, template rendering
  Reqs/sec      9227.84    1106.73   12175.45
  Latency       10.85ms     2.67ms    32.94ms
  Latency Distribution
     50%    10.68ms
     75%    12.55ms
     90%    14.29ms
     99%    19.96ms

## Django Ninja-style Benchmarks
### JSON Parse/Validate (POST /bench/parse)
  Reqs/sec    119492.18    6024.26  127530.79
  Latency      806.73us   509.06us     6.39ms
  Latency Distribution
     50%   670.00us
     75%     0.97ms
     90%     1.38ms
     99%     3.56ms

## Serializer Performance Benchmarks
### Raw msgspec Serializer (POST /bench/serializer-raw)
  Reqs/sec     82846.33    6526.62   89252.96
  Latency        1.18ms   515.31us     6.69ms
  Latency Distribution
     50%     1.06ms
     75%     1.40ms
     90%     1.95ms
     99%     3.55ms
### Django-Bolt Serializer with Validators (POST /bench/serializer-validated)
  Reqs/sec     66012.35    9387.13   80654.99
  Latency        1.48ms   731.52us     5.86ms
  Latency Distribution
     50%     1.26ms
     75%     1.78ms
     90%     2.57ms
     99%     4.83ms
### Users msgspec Serializer (POST /users/bench/msgspec)
  Reqs/sec     79881.92    7163.46   87174.05
  Latency        1.24ms   573.72us     7.06ms
  Latency Distribution
     50%     1.11ms
     75%     1.51ms
     90%     2.06ms
     99%     3.83ms

## Multi-Response Performance

### Multi-response tuple return (/bench/multi/tuple)
  Reqs/sec     85456.26    9237.78   95298.77
  Latency        1.15ms   501.88us     5.63ms
  Latency Distribution
     50%     1.03ms
     75%     1.41ms
     90%     1.90ms
     99%     3.53ms

### Multi-response bare dict (/bench/multi/dict)
  Reqs/sec     81819.06    7294.37   88402.44
  Latency        1.20ms   487.90us     7.93ms
  Latency Distribution
     50%     1.07ms
     75%     1.52ms
     90%     1.99ms
     99%     3.40ms

## Latency Percentile Benchmarks
Measures p50/p75/p90/p99 latency for type coercion overhead analysis

### Baseline - No Parameters (/)
  Reqs/sec    138343.01   15754.16  147895.80
  Latency      712.42us   457.15us     6.17ms
  Latency Distribution
     50%   601.00us
     75%     0.86ms
     90%     1.23ms
     99%     2.98ms

### Path Parameter - int (/items/12345)
  Reqs/sec    114337.70   13204.60  125601.08
  Latency      825.32us   461.54us     5.46ms
  Latency Distribution
     50%   699.00us
     75%     0.98ms
     90%     1.41ms
     99%     3.21ms

### Path + Query Parameters (/items/12345?q=hello)
  Reqs/sec    114191.86   19890.43  132753.27
  Latency        0.85ms   560.01us     9.14ms
  Latency Distribution
     50%   693.00us
     75%     1.05ms
     90%     1.53ms
     99%     3.42ms

### Header Parameter (/header)
  Reqs/sec     76680.39   18488.07   91340.93
  Latency        1.20ms   624.75us     7.39ms
  Latency Distribution
     50%     1.05ms
     75%     1.48ms
     90%     1.99ms
     99%     4.23ms

### Cookie Parameter (/cookie)
  Reqs/sec     82210.12    7870.16   90698.99
  Latency        1.19ms   502.47us     5.33ms
  Latency Distribution
     50%     1.07ms
     75%     1.49ms
     90%     2.08ms
     99%     3.35ms

### Auth Context - JWT validated, no DB (/auth/context)
  Reqs/sec     70437.95    6515.36   77185.01
  Latency        1.39ms   558.72us     6.37ms
  Latency Distribution
     50%     1.26ms
     75%     1.80ms
     90%     2.37ms
     99%     3.59ms
