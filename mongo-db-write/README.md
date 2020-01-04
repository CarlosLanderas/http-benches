
Benchmark results using:


bombardier --method=POST -n 100000 -c 125 http://localhost:8080/books

## AspNetCore

Built with dotnet build -C Release

:clock1: Statistics        Avg      Stdev        Max

  Reqs/sec     21804.69    2632.89   36775.64

  Latency        5.75ms     4.37ms    92.70ms

  HTTP codes:

    1xx - 0, 2xx - 100000, 3xx - 0, 4xx - 0, 5xx - 0

    others - 0

  &#x1F53C; Throughput:     **5.88MB/s**




## Rust Actix Web

Built with cargo build --release

:clock1: Statistics        Avg      Stdev        Max

  Reqs/sec     20285.40    2093.34   22217.26

  Latency        6.15ms     1.55ms    53.05ms

  HTTP codes:

    1xx - 0, 2xx - 100000, 3xx - 0, 4xx - 0, 5xx - 0

    others - 0

  &#x1F53C; Throughput:     **5.30MB/s**



## Golang http

Built with go build -ldflags "-s -w"

:clock1: Statistics        Avg      Stdev        Max

  Reqs/sec     23724.98    6870.73   29740.66

  Latency        5.27ms    11.58ms   421.52ms

  HTTP codes:

    1xx - 0, 2xx - 100000, 3xx - 0, 4xx - 0, 5xx - 0

    others - 0

  &#x1F53C; Throughput:     **6.33MB/s**





