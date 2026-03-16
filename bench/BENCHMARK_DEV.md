# Django-Bolt Benchmark
Generated: Mon 16 Mar 2026 03:26:43 PM PKT
Config: 8 processes × 1 workers | C=100 N=10000

## Root Endpoint Performance
  Reqs/sec    155771.54   12858.48  170292.73
  Latency      601.67us   341.22us     4.93ms
  Latency Distribution
     50%   524.00us
     75%   725.00us
     90%     0.97ms
     99%     2.33ms

## 10kb JSON Response Performance
### 10kb JSON (Async) (/10k-json)
  Reqs/sec    128461.53   23943.55  167649.00
  Latency      812.52us   353.39us     6.45ms
  Latency Distribution
     50%   754.00us
     75%     0.97ms
     90%     1.28ms
     99%     2.01ms
### 10kb JSON (Sync) (/sync-10k-json)
  Reqs/sec    113718.54    9631.32  124362.32
  Latency      831.88us   366.46us     6.04ms
  Latency Distribution
     50%   735.00us
     75%     0.98ms
     90%     1.32ms
     99%     2.75ms

## Response Type Endpoints
### Header Endpoint (/header)
  Reqs/sec    101377.74    8644.65  107155.65
  Latency        0.97ms   317.54us     4.88ms
  Latency Distribution
     50%     0.91ms
     75%     1.18ms
     90%     1.46ms
     99%     2.20ms
### Cookie Endpoint (/cookie)
  Reqs/sec     98508.90    6952.40  104317.89
  Latency        0.99ms   392.25us     5.96ms
  Latency Distribution
     50%     0.90ms
     75%     1.25ms
     90%     1.64ms
     99%     2.65ms
### Exception Endpoint (/exc)
  Reqs/sec    128337.58   10132.41  139082.94
  Latency      760.31us   296.63us     4.36ms
  Latency Distribution
     50%   699.00us
     75%     0.96ms
     90%     1.22ms
     99%     2.10ms
### HTML Response (/html)
  Reqs/sec    152812.53   11454.19  162880.41
  Latency      640.77us   388.21us     5.65ms
  Latency Distribution
     50%   562.00us
     75%   733.00us
     90%     0.99ms
     99%     2.52ms
### Redirect Response (/redirect)
### File Static via FileResponse (/file-static)
  Reqs/sec     36723.80    8212.98   41504.92
  Latency        2.71ms     1.44ms    20.24ms
  Latency Distribution
     50%     2.40ms
     75%     3.08ms
     90%     4.07ms
     99%     9.39ms

## Authentication & Authorization Performance
### Auth NO User Access (/auth/no-user-access) - lazy loading, no DB query
  Reqs/sec     77714.77    5756.96   81647.16
  Latency        1.26ms   336.33us     6.76ms
  Latency Distribution
     50%     1.19ms
     75%     1.51ms
     90%     1.81ms
     99%     2.59ms
### Get Authenticated User (/auth/me) - accesses request.user, triggers DB query
  Reqs/sec     17509.86    1427.92   19805.89
  Latency        5.69ms     1.73ms    15.55ms
  Latency Distribution
     50%     5.63ms
     75%     6.96ms
     90%     8.20ms
     99%    11.17ms
### Get User via Dependency (/auth/me-dependency)
  Reqs/sec     15713.16     889.40   16706.89
  Latency        6.32ms     1.90ms    16.85ms
  Latency Distribution
     50%     6.21ms
     75%     7.76ms
     90%     9.23ms
     99%    12.07ms
### Get Auth Context (/auth/context) validated jwt no db
  Reqs/sec     83717.96    6149.60   89231.25
  Latency        1.17ms   386.10us     5.52ms
  Latency Distribution
     50%     1.08ms
     75%     1.45ms
     90%     1.86ms
     99%     2.75ms

## Items GET Performance (/items/1?q=hello)
  Reqs/sec    142755.35    7897.19  152000.44
  Latency      678.60us   446.05us     7.29ms
  Latency Distribution
     50%   587.00us
     75%   765.00us
     90%     0.98ms
     99%     2.90ms

## Items PUT JSON Performance (/items/1)
  Reqs/sec    146318.67   11715.80  155905.81
  Latency      670.20us   351.52us     6.67ms
  Latency Distribution
     50%   639.00us
     75%   775.00us
     90%     0.90ms
     99%     1.69ms

## ORM Performance
Seeding 1000 users for benchmark...
Successfully seeded users
Validated: 10 users exist in database
### Users Full10 (Async) (/users/full10)
 9299 / 10000 [============================================================================>-----]  92.99% 15473/s
  Reqs/sec     15324.58    1977.72   17301.63
  Latency        6.38ms     1.85ms    17.62ms
  Latency Distribution
     50%     6.33ms
     75%     7.74ms
     90%     9.07ms
     99%    12.42ms
### Users Full10 (Sync) (/users/sync-full10)
  Reqs/sec     12007.49    1727.85   14370.92
  Latency        8.21ms     3.05ms    25.31ms
  Latency Distribution
     50%     7.57ms
     75%     9.98ms
     90%    12.79ms
     99%    18.33ms
### Users Mini10 (Async) (/users/mini10)
  Reqs/sec     18838.92    1143.66   21540.25
  Latency        5.29ms     1.58ms    14.49ms
  Latency Distribution
     50%     5.00ms
     75%     6.21ms
     90%     7.71ms
     99%    11.00ms
### Users Mini10 (Sync) (/users/sync-mini10)
  Reqs/sec     12745.82    1290.33   15650.83
  Latency        7.81ms     3.12ms    27.38ms
  Latency Distribution
     50%     7.07ms
     75%     9.51ms
     90%    12.66ms
     99%    18.18ms
Cleaning up test users...

## Class-Based Views (CBV) Performance
### Simple APIView GET (/cbv-simple)
  Reqs/sec    103724.67    4979.02  106954.22
  Latency        0.95ms   347.89us     5.11ms
  Latency Distribution
     50%     0.87ms
     75%     1.17ms
     90%     1.50ms
     99%     2.36ms
### Simple APIView POST (/cbv-simple)
  Reqs/sec    102639.61    6338.48  108627.44
  Latency        0.96ms   347.08us     6.09ms
  Latency Distribution
     50%     0.89ms
     75%     1.18ms
     90%     1.51ms
     99%     2.38ms
### Items100 ViewSet GET (/cbv-items100)
  Reqs/sec     67564.76    4878.40   71102.30
  Latency        1.46ms   423.20us     4.52ms
  Latency Distribution
     50%     1.33ms
     75%     1.74ms
     90%     2.22ms
     99%     3.27ms

## CBV Items - Basic Operations
### CBV Items GET (Retrieve) (/cbv-items/1)
  Reqs/sec    103826.00    7466.91  110382.91
  Latency        0.94ms   292.31us     4.97ms
  Latency Distribution
     50%     0.89ms
     75%     1.15ms
     90%     1.45ms
     99%     2.17ms
### CBV Items PUT (Update) (/cbv-items/1)
  Reqs/sec     99802.30    4817.34  103521.79
  Latency        0.98ms   309.59us     5.03ms
  Latency Distribution
     50%     0.91ms
     75%     1.23ms
     90%     1.59ms
     99%     2.35ms

## CBV Additional Benchmarks
### CBV Bench Parse (POST /cbv-bench-parse)
  Reqs/sec    101423.67   10669.71  111325.06
  Latency        0.96ms   322.60us     6.35ms
  Latency Distribution
     50%     0.88ms
     75%     1.19ms
     90%     1.53ms
     99%     2.37ms
### CBV Response Types (/cbv-response)
  Reqs/sec    102200.98    4070.85  105314.45
  Latency        0.96ms   327.56us     3.97ms
  Latency Distribution
     50%     0.88ms
     75%     1.20ms
     90%     1.56ms
     99%     2.46ms

## ORM Performance with CBV
Seeding 1000 users for CBV benchmark...
Successfully seeded users
Validated: 10 users exist in database
### Users CBV Mini10 (List) (/users/cbv-mini10)
  Reqs/sec     17236.99    1287.87   19075.85
  Latency        5.76ms     1.76ms    14.66ms
  Latency Distribution
     50%     5.60ms
     75%     7.22ms
     90%     8.54ms
     99%    11.16ms
Cleaning up test users...


## Form and File Upload Performance
### Form Data (POST /form)
  Reqs/sec    129160.13   10000.60  138261.19
  Latency      753.55us   287.88us     5.11ms
  Latency Distribution
     50%   731.00us
     75%     0.94ms
     90%     1.16ms
     99%     1.83ms
### File Upload (POST /upload)
  Reqs/sec    108719.11    8411.69  116055.79
  Latency        0.90ms   315.26us     5.75ms
  Latency Distribution
     50%     0.85ms
     75%     1.11ms
     90%     1.39ms
     99%     2.20ms
### Mixed Form with Files (POST /mixed-form)
  Reqs/sec    105409.04    6277.63  109799.53
  Latency        0.93ms   415.00us     5.27ms
  Latency Distribution
     50%     0.85ms
     75%     1.09ms
     90%     1.40ms
     99%     3.23ms

## Django Middleware Performance
### Django Middleware + Messages Framework (/middleware/demo)
Tests: SessionMiddleware, AuthenticationMiddleware, MessageMiddleware, custom middleware, template rendering
  Reqs/sec     10005.10    1202.47   14670.54
  Latency       10.03ms     2.08ms    23.12ms
  Latency Distribution
     50%     9.71ms
     75%    11.38ms
     90%    12.90ms
     99%    16.83ms

## Django Ninja-style Benchmarks
### JSON Parse/Validate (POST /bench/parse)
  Reqs/sec    135453.52    8868.71  141808.84
  Latency      715.82us   372.58us     6.08ms
  Latency Distribution
     50%   665.00us
     75%   836.00us
     90%     1.04ms
     99%     2.13ms

## Serializer Performance Benchmarks
### Raw msgspec Serializer (POST /bench/serializer-raw)
  Reqs/sec    103210.94    9518.79  110491.12
  Latency        0.95ms   319.71us     4.89ms
  Latency Distribution
     50%     0.88ms
     75%     1.16ms
     90%     1.47ms
     99%     2.38ms
### Django-Bolt Serializer with Validators (POST /bench/serializer-validated)
  Reqs/sec     92969.20    8122.95  100416.01
  Latency        1.06ms   353.80us     4.46ms
  Latency Distribution
     50%     0.99ms
     75%     1.28ms
     90%     1.65ms
     99%     2.70ms
### Users msgspec Serializer (POST /users/bench/msgspec)
  Reqs/sec     99157.81   12301.77  107300.52
  Latency        0.98ms   315.04us     5.87ms
  Latency Distribution
     50%     0.92ms
     75%     1.21ms
     90%     1.50ms
     99%     2.24ms

## Multi-Response Performance

### Multi-response tuple return (/bench/multi/tuple)
  Reqs/sec    105213.65    7436.41  112985.43
  Latency        0.93ms   354.79us     5.11ms
  Latency Distribution
     50%     0.86ms
     75%     1.14ms
     90%     1.42ms
     99%     2.37ms

### Multi-response bare dict (/bench/multi/dict)
  Reqs/sec    106107.45    8310.09  112789.92
  Latency        0.92ms   319.15us     5.24ms
  Latency Distribution
     50%     0.86ms
     75%     1.15ms
     90%     1.48ms
     99%     2.24ms

## Latency Percentile Benchmarks
Measures p50/p75/p90/p99 latency for type coercion overhead analysis

### Baseline - No Parameters (/)
  Reqs/sec    163092.44   15858.42  173788.32
  Latency      596.26us   300.62us     5.79ms
  Latency Distribution
     50%   535.00us
     75%   709.00us
     90%     1.00ms
     99%     1.87ms

### Path Parameter - int (/items/12345)
  Reqs/sec    136175.19   15927.37  150247.86
  Latency      721.25us   409.94us     5.75ms
  Latency Distribution
     50%   637.00us
     75%     0.87ms
     90%     1.14ms
     99%     2.94ms

### Path + Query Parameters (/items/12345?q=hello)
  Reqs/sec    141549.99   15202.76  151016.32
  Latency      694.24us   341.08us     5.15ms
  Latency Distribution
     50%   629.00us
     75%   826.00us
     90%     1.11ms
     99%     2.38ms

### Header Parameter (/header)
  Reqs/sec     99896.34    9020.69  108249.75
  Latency        0.98ms   406.96us     5.50ms
  Latency Distribution
     50%     0.89ms
     75%     1.20ms
     90%     1.58ms
     99%     2.99ms

### Cookie Parameter (/cookie)
  Reqs/sec    100215.49    6769.32  105883.56
  Latency        0.98ms   384.72us     4.91ms
  Latency Distribution
     50%     0.88ms
     75%     1.20ms
     90%     1.56ms
     99%     2.96ms

### Auth Context - JWT validated, no DB (/auth/context)
  Reqs/sec     82648.12    7604.22   89057.54
  Latency        1.18ms   426.34us     5.89ms
  Latency Distribution
     50%     1.10ms
     75%     1.46ms
     90%     1.88ms
     99%     3.15ms
