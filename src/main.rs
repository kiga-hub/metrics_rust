use prometheus::{GaugeVec, Opts, register_gauge_vec};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use prometheus::{Encoder, TextEncoder};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::io::{self, Write};


// an asynchronous function that handles incoming HTTP requests. 
// It gathers all the metrics registered with the Prometheus client
// encodes them into the Prometheus exposition format, and returns them in the HTTP response.
async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    Ok(Response::new(Body::from(buffer)))
}

// starts an HTTP server that listens on a given address. 
// it uses the serve_req function to handle incoming requests.
async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);

    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(serve_req)) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

// This struct holds two GaugeVec metrics, current and target. 
// A GaugeVec is a Prometheus metric type that represents a set of key-value pairs and a floating-point number.
struct Metrics {
    current: GaugeVec,
    target: GaugeVec,
}

impl Metrics {
    // This method creates a new Metrics instance. It registers the current and target metrics with the Prometheus client.
    fn new() -> Metrics {
        let current = register_gauge_vec!(
            Opts::new("current", "current value")
                .namespace("metrics")
                .subsystem("rust"),
            &["name", "lebel"]
        ).unwrap();

        let target = register_gauge_vec!(
            Opts::new("target", "target current value")
                .namespace("metrics")
                .subsystem("rust"),
            &["name"]
        ).unwrap();

        Metrics { current, target }
    }
}

#[tokio::main]
async fn main() {
    let metrics = Metrics::new();
    let name = "This is a test";
    let strx = "labels name";

    let addr = SocketAddr::from(([192, 168, 8, 244], 7777));
    tokio::spawn(run_server(addr));

    println!("Start");

    loop {
        let mut input = String::new();
        print!("Enter current and target (or 'q' to quit): ");
        io::stdout().flush().unwrap();  // Flush stdout to display the prompt before blocking on input
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "q" {
            break;
        }

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() < 2 {
            println!("Please enter both current and target");
            continue;
        }

        let current: f64 = parts[0].parse().expect("Failed to parse current");
        let target: f64 = parts[1].parse().expect("Failed to parse target");

        println!("current: {}, Target: {}", current, target);

        metrics.current.with_label_values(&[name, strx]).set(current);
        metrics.target.with_label_values(&[name]).set(target);
    }
}