# Basic OTLP exporter Example

## 1. Setup and run ADOT:
- Follow these steps https://aws-otel.github.io/docs/setup/ec2 and setup ADOT on a metal instance.
- Run ```sudo /opt/aws/aws-otel-collector/bin/aws-otel-collector-ctl -a status``` and confirm it is running:
```
{
  "status": "running",
  "starttime": "2023-08-21T14:54:29+0000",
  "version": "v0.32.0"
}
```
  
## 2. Setup and run OTLP example:
-
  ```
     cd ~
     git clone https://github.com/open-telemetry/opentelemetry-rust.git
     git clone https://github.com/sudanl0/test/
     cd test/simple_otlp_adot/
     cargo run
  ```

## 3. View result
`CloudWatch -> Metrics` will have a new entry in `All -> basic-otlp-metrics-example` which can be used to view metric in graphs.
and
CLoudwatch `CloudWatch -> Log groups -> /metrics/basic-otlp-metrics-example` will show metrics as below:


```
{
    "NetDeviceMetrics": 0,
    "OTelLib": "fc-meter",
    "Version": "1",
    "_aws": {
        "CloudWatchMetrics": [
            {
                "Namespace": "basic-otlp-metrics-example",
                "Dimensions": [
                    [
                        "dev1",
                        "total",
                        "OTelLib"
                    ],
                    [
                        "OTelLib"
                    ],
                    [
                        "OTelLib",
                        "dev1"
                    ],
                    [
                        "OTelLib",
                        "total"
                    ]
                ],
                "Metrics": [
                    {
                        "Name": "NetDeviceMetrics",
                        "Unit": "Bytes"
                    },
                    {
                        "Name": "vsock",
                        "Unit": "Bytes"
                    }
                ]
            }
        ],
        "Timestamp": 1692631168928
    },
    "dev1": "rx_bytes_count",
    "total": "rx_bytes_count",
    "vsock": 0
}
{
    "NetDeviceMetrics": 0,
    "OTelLib": "fc-meter",
    "Version": "1",
    "_aws": {
        "CloudWatchMetrics": [
            {
                "Namespace": "basic-otlp-metrics-example",
                "Dimensions": [
                    [
                        "OTelLib"
                    ],
                    [
                        "OTelLib",
                        "dev1"
                    ]
                ],
                "Metrics": [
                    {
                        "Name": "NetDeviceMetrics",
                        "Unit": "Bytes"
                    },
                    {
                        "Name": "vsock",
                        "Unit": "Bytes"
                    }
                ]
            }
        ],
        "Timestamp": 1692631168928
    },
    "dev1": "tx_bytes_count",
    "vsock": 0
}
{
    "NetDeviceMetrics": 0,
    "OTelLib": "fc-meter",
    "Version": "1",
    "_aws": {
        "CloudWatchMetrics": [
            {
                "Namespace": "basic-otlp-metrics-example",
                "Dimensions": [
                    [
                        "OTelLib"
                    ],
                    [
                        "OTelLib",
                        "dev0"
                    ]
                ],
                "Metrics": [
                    {
                        "Name": "NetDeviceMetrics",
                        "Unit": "Bytes"
                    },
                    {
                        "Name": "vsock",
                        "Unit": "Bytes"
                    }
                ]
            }
        ],
        "Timestamp": 1692631168928
    },
    "dev0": "tx_bytes_count",
    "vsock": 0
}
{
    "NetDeviceMetrics": 0,
    "OTelLib": "fc-meter",
    "Version": "1",
    "_aws": {
        "CloudWatchMetrics": [
            {
                "Namespace": "basic-otlp-metrics-example",
                "Dimensions": [
                    [
                        "dev0",
                        "total",
                        "OTelLib"
                    ],
                    [
                        "OTelLib"
                    ],
                    [
                        "OTelLib",
                        "dev0"
                    ],
                    [
                        "OTelLib",
                        "total"
                    ]
                ],
                "Metrics": [
                    {
                        "Name": "NetDeviceMetrics",
                        "Unit": "Bytes"
                    },
                    {
                        "Name": "vsock",
                        "Unit": "Bytes"
                    }
                ]
            }
        ],
        "Timestamp": 1692631168928
    },
    "dev0": "rx_bytes_count",
    "total": "rx_bytes_count",
    "vsock": 0
}
{
    "NetDeviceMetrics": 0,
    "OTelLib": "fc-meter",
    "Version": "1",
    "_aws": {
        "CloudWatchMetrics": [
            {
                "Namespace": "basic-otlp-metrics-example",
                "Dimensions": [
                    [
                        "dev1",
                        "total",
                        "OTelLib"
                    ],
                    [
                        "OTelLib"
                    ],
                    [
                        "OTelLib",
                        "dev1"
                    ],
                    [
                        "OTelLib",
                        "total"
                    ]
                ],
                "Metrics": [
                    {
                        "Name": "NetDeviceMetrics",
                        "Unit": "Bytes"
                    },
                    {
                        "Name": "vsock",
                        "Unit": "Bytes"
                    }
                ]
            }
        ],
        "Timestamp": 1692631169175
    },
    "dev1": "rx_bytes_count",
    "total": "rx_bytes_count",
    "vsock": 0
}
{
    "NetDeviceMetrics": 0,
    "OTelLib": "fc-meter",
    "Version": "1",
    "_aws": {
        "CloudWatchMetrics": [
            {
                "Namespace": "basic-otlp-metrics-example",
                "Dimensions": [
                    [
                        "OTelLib"
                    ],
                    [
                        "OTelLib",
                        "dev1"
                    ]
                ],
                "Metrics": [
                    {
                        "Name": "NetDeviceMetrics",
                        "Unit": "Bytes"
                    },
                    {
                        "Name": "vsock",
                        "Unit": "Bytes"
                    }
                ]
            }
        ],
        "Timestamp": 1692631169175
    },
    "dev1": "tx_bytes_count",
    "vsock": 0
}
{
    "NetDeviceMetrics": 0,
    "OTelLib": "fc-meter",
    "Version": "1",
    "_aws": {
        "CloudWatchMetrics": [
            {
                "Namespace": "basic-otlp-metrics-example",
                "Dimensions": [
                    [
                        "OTelLib"
                    ],
                    [
                        "OTelLib",
                        "dev0"
                    ]
                ],
                "Metrics": [
                    {
                        "Name": "NetDeviceMetrics",
                        "Unit": "Bytes"
                    },
                    {
                        "Name": "vsock",
                        "Unit": "Bytes"
                    }
                ]
            }
        ],
        "Timestamp": 1692631169175
    },
    "dev0": "tx_bytes_count",
    "vsock": 0
}
{
    "NetDeviceMetrics": 0,
    "OTelLib": "fc-meter",
    "Version": "1",
    "_aws": {
        "CloudWatchMetrics": [
            {
                "Namespace": "basic-otlp-metrics-example",
                "Dimensions": [
                    [
                        "OTelLib",
                        "dev0",
                        "total"
                    ],
                    [
                        "OTelLib"
                    ],
                    [
                        "OTelLib",
                        "dev0"
                    ],
                    [
                        "OTelLib",
                        "total"
                    ]
                ],
                "Metrics": [
                    {
                        "Name": "vsock",
                        "Unit": "Bytes"
                    },
                    {
                        "Name": "NetDeviceMetrics",
                        "Unit": "Bytes"
                    }
                ]
            }
        ],
        "Timestamp": 1692631169175
    },
    "dev0": "rx_bytes_count",
    "total": "rx_bytes_count",
    "vsock": 0
}
```
