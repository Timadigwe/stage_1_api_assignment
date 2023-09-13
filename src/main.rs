use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response};
use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use serde_urlencoded;
use std::net::SocketAddr;

#[derive(Debug, Deserialize)]
struct QueryParams {
    slack_name: String,
    track: String,
}

#[derive(Serialize)]
struct ApiResponse {
    slack_name: String,
    current_day: String,
    utc_time: String,
    track: String,
    github_file_url: String,
    github_repo_url: String,
    status_code: u16,
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Parse query parameters
    let query = req.uri().query().unwrap_or("");
    let query_params: QueryParams = serde_urlencoded::from_str(query).unwrap_or_default();

    // Get current day and time
    let current_day = Local::now().format("%A").to_string();
    let utc_time = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

    // Create JSON response
    let response = ApiResponse {
        slack_name: query_params.slack_name,
        current_day,
        utc_time,
        track: query_params.track,
        github_file_url: "https://github.com/username/repo/blob/main/file_name.ext".to_string(),
        github_repo_url: "https://github.com/username/repo".to_string(),
        status_code: 200,
    };

    // Serialize the response to JSON
    let response_json = serde_json::to_string(&response).unwrap();

    Ok(Response::new(Body::from(response_json)))
}

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    // Create a service to handle requests
    let make_svc = make_service_fn(|_conn| {
        let service = service_fn(handle_request);
        async { Ok::<_, hyper::Error>(service) }
    });

    // Create a server and bind it to the specified address
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}