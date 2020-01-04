
Benchmark results using:


bombardier --method=POST -n 100 -n 25 http://localhost:8080/call

## AspNetCore

Built with dotnet build -C Release




## Rust Actix Web

Built with cargo build --release




## Golang http

Built with go build -ldflags "-s -w"

:clock1: Statistics        Avg      Stdev        Max

  Reqs/sec       161.03     367.46    2050.09

  Latency      504.78ms    55.10ms   628.09ms

  HTTP codes:

    1xx - 0, 2xx - 100, 3xx - 0, 4xx - 0, 5xx - 0

    others - 0


  &#x1F53C; Throughput:     **46.95KB/s**





