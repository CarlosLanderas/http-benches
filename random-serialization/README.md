
Benchmark results using:

bombardier -c 125 -n 1000000 http://localhost:8080/user

## AspNetCore

Built with dotnet build -c Release

:clock1: Statistics        Avg      Stdev        Max

  Reqs/sec    133441.23   14902.67  181375.34

  Latency        0.93ms     1.01ms    59.04ms

  HTTP codes:

    1xx - 0, 2xx - 1000000, 3xx - 0, 4xx - 0, 5xx - 0

    others - 0

  &#x1F53C; Throughput:    **33.34MB/s**



## Rust Actix Web

Built with cargo build --release


:clock1: Statistics        Avg      Stdev        Max

  Reqs/sec    218368.40   15628.36  279702.25

  Latency      569.33us   711.96us    41.17ms

  HTTP codes:

    1xx - 0, 2xx - 1000000, 3xx - 0, 4xx - 0, 5xx - 0

    others - 0

  &#x1F53C; Throughput:    **53.95MB/s**


## Golang http

Built with go build -ldflags "-s -w"

:clock1: Statistics        Avg      Stdev        Max

  Reqs/sec    123880.18    5752.52  163997.09
  
  Latency        1.01ms   452.26us    18.29ms
  
  HTTP codes:
  
    1xx - 0, 2xx - 1000000, 3xx - 0, 4xx - 0, 5xx - 0
    
    others - 0
    
  &#x1F53C; Throughput:    **29.54MB/s**





