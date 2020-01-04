Benchmark results using:


bombardier --method=POST -n 100000 -c 125 http://localhost:8080/books

## AspNetCore

Built with dotnet build -C Release

:clock1: Statistics        Avg      Stdev        Max

  Reqs/sec     26827.00    7341.45   43169.99

  Latency        4.66ms     4.33ms   118.83ms

  HTTP codes:

    1xx - 0, 2xx - 100000, 3xx - 0, 4xx - 0, 5xx - 0

    others - 0

  &#x1F53C; Throughput:  **7.23MB/s**


## Rust Actix Web

Built with cargo build --release



## Golang http

Built with go build -ldflags "-s -w"

:clock1: Statistics        Avg      Stdev        Max

  Reqs/sec     33266.34   11584.12   57608.95

  Latency        3.76ms     6.63ms    92.67ms

  HTTP codes:

    1xx - 0, 2xx - 100000, 3xx - 0, 4xx - 0, 5xx - 0

    others - 0

  &#x1F53C; Throughput: **9.12MB/s**













