# metrics_rust

This Rust program is a simple server that collects and exposes metrics using the Prometheus library. The metrics are exposed over HTTP and can be scraped by a Prometheus server.

The metrics are labeled with a name and an additional label for the current metric. 

The labels are static in this example, but in a real-world application, they could be dynamic to provide more context about the metric.

## Dependencies

This program uses the following dependencies:

[`prometheus`]: This is a Rust client for the Prometheus monitoring system.

[`tokio`] A Rust framework for developing applications with asynchronous I/O, mainly networked applications
- [`tokio::main`] An attribute used to mark the entry point of the application
- [`tokio::spawn`] A function used to start the server in a new task.

[`std::io`]: A part of the Rust standard library and provides a number of useful features for handling input/output. used to read user input

[`stdout`] A function used to print to the console.

[`std::net::SocketAddr`]: A part of the Rust standard library and provides a type used to represent a socket address.

[`hyper`]: A fast and correct HTTP library for Rust.


## deployment

```bash
docker pull prom/prometheus
docker pull grafana/grafana
docker pull prom/alertmanager
docker pull mariadb:10.5.4
```

```bash
docker-compose -f docker-compose.yaml up -d
```

run meterics 

```bash
cargo run
# input current and target. (e.g. 15.9 15)
```
