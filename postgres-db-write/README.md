
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

  &#x1F53C; Throughput:     **1.36MB/s**



## Rust Actix Web

Built with cargo build --release

Statistics        Avg      Stdev        Max

  Reqs/sec      4792.27     767.33   22592.74

  Latency       26.16ms     9.43ms   146.68ms

  HTTP codes:

    1xx - 0, 2xx - 100000, 3xx - 0, 4xx - 0, 5xx - 0

    others - 0

   &#x1F53C; Throughput:     **1.17MB/s**




## Golang http

Built with go build -ldflags "-s -w"

Statistics        Avg      Stdev        Max

  Reqs/sec      7732.64    1578.78   10715.33

  Latency       16.13ms     6.21ms   142.05ms

  HTTP codes:

    1xx - 0, 2xx - 100000, 3xx - 0, 4xx - 0, 5xx - 0

    others - 0

   &#x1F53C; Throughput:     **1.89MB/s**




