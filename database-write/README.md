
Benchmark results using:

bombardier --method=POST -c 125 -n 100000 http://localhost:8080/user

## AspNetCore

Built with dotnet build -c Release

Statistics        Avg      Stdev        Max

  Reqs/sec      5129.50    1696.08   10090.54

  Latency       24.36ms    42.04ms      1.23s

  HTTP codes:

    1xx - 0, 2xx - 100000, 3xx - 0, 4xx - 0, 5xx - 0

    others - 0

  Throughput:     **1.23MB/s**


## Rust Actix Web

Built with cargo build --release

Statistics        Avg      Stdev        Max

  Reqs/sec      4609.69     569.11    5302.21

  Latency       27.08ms    10.29ms   177.36ms

  HTTP codes:

    1xx - 0, 2xx - 100000, 3xx - 0, 4xx - 0, 5xx - 0

    others - 0

  Throughput:     **1.13MB/s**



## Golang http





