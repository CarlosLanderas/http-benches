
Benchmark results using:

bombardier --method=POST -n 100000 -c 125 http://localhost:8080/books

## AspNetCore



## Rust Actix Web

Built with cargo build --release

Statistics        Avg      Stdev        Max

  Reqs/sec     20285.40    2093.34   22217.26

  Latency        6.15ms     1.55ms    53.05ms

  HTTP codes:

    1xx - 0, 2xx - 100000, 3xx - 0, 4xx - 0, 5xx - 0

    others - 0

  Throughput:     **5.30MB/s**



## Golang http




