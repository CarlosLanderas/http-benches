
Benchmark results using:

bombardier --method=POST -c 125 -n 100000 http://localhost:8080/user

## AspNetCore

Built with dotnet build -c Release

Statistics        Avg      Stdev        Max

  Reqs/sec      5271.47    1478.68   14494.03

  Latency       23.74ms     5.55ms   273.69ms

  HTTP codes:

    1xx - 0, 2xx - 99979, 3xx - 0, 4xx - 0, 5xx - 21

    others - 0

  Throughput:     **1.26MB/s**




## Rust Actix Web

Statistics        Avg      Stdev        Max

  Reqs/sec      4609.69     569.11    5302.21

  Latency       27.08ms    10.29ms   177.36ms

  HTTP codes:

    1xx - 0, 2xx - 100000, 3xx - 0, 4xx - 0, 5xx - 0

    others - 0

  Throughput:     1.13MB/s



## Golang http





