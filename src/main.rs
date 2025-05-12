use axum::{routing::post, Router, Json};
use axum::http::StatusCode;
use reqwest::Client;
use std::error::Error;
use tower_http::cors::{CorsLayer, Any};

fn configure_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin("https://upwork-frontend-ten.vercel.app".parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/generate", post(|body: String| async move { generate_text(body).await }))
        .layer(configure_cors());

    Ok(router.into())
}

async fn generate_text(
    body: String,
) -> Result<Json<String>, (StatusCode, String)> {
    match call_gemini_ai_studio(body).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn call_gemini_ai_studio(
    prompt: String,
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let api_url = std::env::var("GEMINI_API_URL").expect("GEMINI_API_URL environment variable not set");
    let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");

    let response = client
        .post(api_url)
        .query(&[("key", api_key)]) // Pass the API key as a query parameter
        .json(&serde_json::json!({
            "contents": [
                {
                    "parts": [
                        {
                            "text": prompt
                        }
                    ]
                }
            ]
        }))
        .send()
        .await?;

    if response.status().is_success() {
        let response_json: serde_json::Value = response.json().await?;
        if let Some(result) = response_json["candidates"]
            .get(0)
            .and_then(|candidate| candidate["content"]["parts"].get(0))
            .and_then(|part| part["text"].as_str())
        {
            Ok(result.to_string())
        } else {
            Err("Invalid response format".into())
        }
    } else {
        let error_message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("API call failed with error: {}", error_message).into())
    }
}
