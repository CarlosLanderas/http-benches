
Benchmark results using:

bombardier --method=POST -c 125 -n 100000 http://localhost:8080/user

## AspNetCore

Built with dotnet build -c Release

Statistics        Avg      Stdev        Max

  Reqs/sec      5657.03    1951.26   10712.28

  Latency       22.08ms    37.79ms      1.11s

  HTTP codes:

    1xx - 0, 2xx - 100000, 3xx - 0, 4xx - 0, 5xx - 0

    others - 0

  Throughput:     **1.36MB/s**



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





