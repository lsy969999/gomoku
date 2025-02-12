use lambda_http::tracing;
use serde_json::{json, Value};

use crate::config::APP_CONFIG;

pub async fn ws_initial(
    connection_id: &str,
    http_client: reqwest::Client,
    jwt: Option<String>,
) -> anyhow::Result<()> {
    // get jwt by connection_id
    let base_url = &APP_CONFIG.get().unwrap().settings.api.url;
    let url = format!("{base_url}/ws/initial");

    let res = http_client
        .post(url)
        .json(&json!({
            "connectionId": connection_id,
            "jwt": jwt
        }))
        .send()
        .await?;
    let j = res.json::<Value>().await?;

    Ok(())
}
