use std::collections::HashMap;

use serde::Serialize;
use serde_json::json;
use url::Url;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[derive(Serialize)]
struct Tool {
    name: String,
    description: String,
    url: String,
}

impl Tool {
    pub fn new(name: &str, description: &str, url: &str) -> Tool {
        Tool {
            name: name.into(),
            description: description.into(),
            url: url.into(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let headers = _req.headers();
    let user_agent = match headers.get("user-agent") {
        Some(value) => value.to_str().unwrap(),
        None => "unknown",
    };

    let url = Url::parse(&_req.uri().to_string()).unwrap();

    let query_params = url
        .query_pairs()
        .into_owned()
        .collect::<HashMap<String, String>>();

    let tools = vec![
        Tool::new("alert", "Test the alert() web API", "alert"),
        Tool::new(
            "calculator",
            "A simple basic calculator",
            "basic-calculator",
        ),
    ];

    let query = query_params
        .get("q")
        .unwrap_or(&"".to_string())
        .to_lowercase();

    let mut results: Vec<Tool> = tools
        .into_iter()
        .filter(|tool| {
            tool.name.to_lowercase().contains(&query.to_lowercase())
                || tool
                    .description
                    .to_lowercase()
                    .contains(&query.to_lowercase())
        })
        .collect();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header(
            "Cache-Control",
            format!(
                "public, max-age=0, must-revalidate, s-maxage={s_maxage}",
                s_maxage = 1 * 60 * 60
            ),
        )
        .body(
            json!({
                "path": url.path(),
                "query": {
                    "term": query_params.get("q"),
                    "results": results.truncate(6)
                },
                "headers": {
                    "userAgent": user_agent
                }
            })
            .to_string()
            .into(),
        )?)
}
