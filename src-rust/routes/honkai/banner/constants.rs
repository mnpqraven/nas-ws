use crate::handler::error::WorkerError;
use semver::Version;
use serde::{Deserialize, Serialize};

// char1, char2, version
pub(super) const BANNER_CHARS: [(&str, Option<&str>, Option<&str>); 3] = [
    ("1.1.0", Some("silverwolf"), Some("luocha")),
    ("1.2.0", Some("blade"), Some("kafka")),
    ("1.3.0", Some("fuxuan"), Some("danhengil")),
];

#[derive(Debug, Serialize, Deserialize)]
struct PatchData {
    version: String,
    banner_data: Vec<BannerData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BannerData {
    phase: u32,
    chara: [Option<u32>; 4],
    lc: [Option<u32>; 4],
}

#[derive(Debug, Serialize, Deserialize)]
struct EdgeResponse {
    #[serde(alias = "createdAt")]
    created_at: u64,
    key: String,
    value: Vec<PatchData>,
}

const EDGE_ID: &str = "ecfg_mf3afhofr3c94tr0gimsibnmo454";

pub(super) async fn get_banner_char2() -> Result<(), WorkerError> {
    let client = reqwest::Client::new();

    let key = "EDGE";
    let t = std::env::var(key);

    if let Ok(token) = std::env::var(key) {
        let url = format!("https://api.vercel.com/v1/edge-config/{}/items", EDGE_ID);

        let res = client
            .get(url)
            .bearer_auth(token)
            .send()
            .await?
            .text()
            .await?;

        let sanitized = if res.starts_with('[') && res.ends_with(']') {
            let mut sanitized = res.trim().chars();
            sanitized.next();
            sanitized.next_back();
            sanitized.as_str()
        } else {
            res.trim()
        };

        let response: EdgeResponse = serde_json::from_str(sanitized)?;

        dbg!(&response);

        // dbg!(&text);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::get_banner_char2;

    #[tokio::test]
    async fn t() {
        get_banner_char2().await.unwrap();
    }
}
