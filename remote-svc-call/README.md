
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

:clock1: Statistics        Avg      Stdev        Max

  Reqs/sec       206.30     260.07    1047.29

  Latency      128.93ms    12.43ms   175.73ms

  HTTP codes:

    1xx - 0, 2xx - 100, 3xx - 0, 4xx - 0, 5xx - 0

    others - 0

  &#x1F53C; Throughput:  **52.81KB/s**


## Golang http

Built with go build -ldflags "-s -w"

:clock1: Statistics        Avg      Stdev        Max

  Reqs/sec       270.59     326.97    1052.25

  Latency       96.75ms   113.73ms   307.86ms

  HTTP codes:

    1xx - 0, 2xx - 100, 3xx - 0, 4xx - 0, 5xx - 0

    others - 0

  &#x1F53C; Throughput:  **73.43KB/s**






