Benchmark results using:


bombardier --method=POST -n 100000 -c 125 http://localhost:8080/file

## AspNetCore

Built with dotnet build -C Release

:clock1: Statistics        Avg      Stdev        Max

  Reqs/sec     30252.67    5526.41   57624.34

  Latency        4.15ms     3.53ms    95.44ms

  HTTP codes:

    1xx - 0, 2xx - 100000, 3xx - 0, 4xx - 0, 5xx - 0

    others - 0

  &#x1F53C; Throughput: **8.11MB/s**



## Rust Actix Web

Built with cargo build --release

:clock1: Statistics        Avg      Stdev        Max

  Reqs/sec     52490.27    3847.50   60946.02

  Latency        2.37ms     2.57ms    34.14ms

  HTTP codes:

    1xx - 0, 2xx - 100000, 3xx - 0, 4xx - 0, 5xx - 0

    others - 0

  &#x1F53C; Throughput: **13.67MB/s**

## Golang http

Built with go build -ldflags "-s -w"

:clock1: Statistics        Avg      Stdev        Max

  Reqs/sec     33266.34   11584.12   57608.95

  Latency        3.76ms     6.63ms    92.67ms

  HTTP codes:

    1xx - 0, 2xx - 100000, 3xx - 0, 4xx - 0, 5xx - 0

    others - 0

  &#x1F53C; Throughput: **9.12MB/s**













