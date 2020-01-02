
Benchmark results using:

bombardier --method=POST -c 125 -n 100000 http://localhost:8080/user

## AspNetCore

Built with dotnet build -c Release

Statistics        Avg      Stdev        Max

  Reqs/sec      1333.65     427.29    3846.11

  Latency       93.61ms    32.85ms   841.00ms

  HTTP codes:

    1xx - 0, 2xx - 0, 3xx - 0, 4xx - 0, 5xx - 100000

    others - 0

  Throughput:   **320.66KB/s**




## Rust Actix Web



## Golang http





