
Benchmark results using:


bombardier --method=POST -n 100 -n 25 http://localhost:8080/call

## AspNetCore

Built with dotnet build -C Release

:clock1: Statistics        Avg      Stdev        Max

  Reqs/sec       253.26     233.73     949.47

  Latency       88.49ms    55.41ms   218.25ms

  HTTP codes:

    1xx - 0, 2xx - 100, 3xx - 0, 4xx - 0, 5xx - 0

    others - 0

&#x1F53C; Throughput:  **81.88KB/s**


## Rust Actix Web

Built with cargo build --release


## Golang http

Built with go build -ldflags "-s -w"

:clock1: Statistics        Avg      Stdev        Max

  Reqs/sec       556.78     419.56    1101.19

  Latency       38.79ms     7.63ms    66.55ms

  HTTP codes:

    1xx - 0, 2xx - 100, 3xx - 0, 4xx - 0, 5xx - 0

    others - 0


  &#x1F53C; Throughput:  **164.48KB/s**





