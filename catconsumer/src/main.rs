use axum::{
    body::Body,
    extract::State,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use reqwest::{Client, StatusCode};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with a route
    let client = Client::new();
    let app = Router::new()
        .route("/", get(reqwest_response))
        .with_state(client);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn reqwest_response(State(client): State<Client>) -> Response {
    let reqwest_response = match client.get("http://127.0.0.1:3001/").send().await {
        Ok(res) => res,
        Err(err) => {
            tracing::error!(%err, "request failed");
            return (StatusCode::BAD_REQUEST, Body::empty()).into_response();
        }
    };

    let mut response_builder = Response::builder().status(reqwest_response.status());
    *response_builder.headers_mut().unwrap() = reqwest_response.headers().clone();
    response_builder
        .body(Body::from(
            reqwest_response
                .text()
                .await
                .unwrap_or("".to_string())
                .to_string(),
        ))
        .unwrap()
}
