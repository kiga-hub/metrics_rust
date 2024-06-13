use prometheus::{GaugeVec, Opts, register_gauge_vec};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use prometheus::{Encoder, TextEncoder};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::io::{self, Write};

async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    Ok(Response::new(Body::from(buffer)))
}

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

struct Metrics {
    thickness: GaugeVec,
    target: GaugeVec,
}

impl Metrics {
    fn new() -> Metrics {
        let thickness = register_gauge_vec!(
            Opts::new("thickness", "thickness value")
                .namespace("app")
                .subsystem("engine"),
            &["name", "x"]
        ).unwrap();

        let target = register_gauge_vec!(
            Opts::new("target", "target thickness value")
                .namespace("app")
                .subsystem("engine"),
            &["name"]
        ).unwrap();

        Metrics { thickness, target }
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
        print!("Enter thickness and target (or 'q' to quit): ");
        io::stdout().flush().unwrap();  // Flush stdout to display the prompt before blocking on input
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "q" {
            break;
        }

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() < 2 {
            println!("Please enter both thickness and target");
            continue;
        }

        let thickness: f64 = parts[0].parse().expect("Failed to parse thickness");
        let target: f64 = parts[1].parse().expect("Failed to parse target");

        println!("Thickness: {}, Target: {}", thickness, target);

        metrics.thickness.with_label_values(&[name, strx]).set(thickness);
        metrics.target.with_label_values(&[name]).set(target);
    }
}