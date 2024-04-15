use anyhow::Result;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use rust_bert::pipelines::sentiment::SentimentModel;
use std::collections::HashMap;
use tokio::task;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let query_params = event.uri().query().unwrap_or("");
    let query_map: HashMap<String, String> =
        serde_urlencoded::from_str(query_params).expect("Invalid query");
    let input_res = match query_map.get("text") {
        Some(input) => input,
        None => {
            let resp = Response::builder()
                .status(500)
                .header("content-type", "text/html")
                .body(("invalid format").into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    };
    let input_data = input_res.clone();
    let res_data = task::spawn_blocking(move || {
        let sentiment_classifier =
            SentimentModel::new(Default::default()).expect("Error creating model");
        sentiment_classifier.predict(&[input_data.as_str()])
    })
    .await
    .map_err(anyhow::Error::new)?;
    println!("{}", serde_json::to_string(&res_data[0].polarity).unwrap());
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(serde_json::to_string(&res_data[0].polarity).unwrap().into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
