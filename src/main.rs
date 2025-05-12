use axum::{routing::post, Router, Json};
use axum::http::StatusCode;
use reqwest::Client;
use std::error::Error;
use tower_http::cors::{CorsLayer};
use shuttle_runtime::SecretStore;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let cors = CorsLayer::very_permissive();

    let router = Router::new()
        .route("/generate", post(|body: String| async move { generate_text(body, secrets).await }))
        .layer(cors);

    Ok(router.into())
}

async fn generate_text(
    body: String,
    secrets: SecretStore
) -> Result<Json<String>, (StatusCode, String)> {
    match call_gemini_ai_studio(body, secrets).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn call_gemini_ai_studio(
    prompt: String,
    secrets: SecretStore
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let api_url = secrets.get("GEMINI_API_URL").expect("GEMINI_API_URL secret not found");
    let api_key = secrets.get("GEMINI_API_KEY").expect("GEMINI_API_KEY secret not found");

    let response = client
        .post(api_url)
        .query(&[("key", api_key)]) // Pass the API key as a query parameter
        .json(&serde_json::json!({
            "contents": [
                {
                    "parts": [
                        {
                            "text": format!("paraphrase this text and return just the result only without any unimportant explanation : {}", prompt),
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
            Ok(result[0..result.len() - 2].to_string())
        } else {
            Err("Invalid response format".into())
        }
    } else {
        let error_message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("API call failed with error: {}", error_message).into())
    }
}
