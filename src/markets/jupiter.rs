use serde::Deserialize;
use tokio::time::{sleep, Duration};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JupiterQuoteResponse {
    out_amount: String,
    other_amount_threshold: String,
}

pub async fn simulate_route_jupiter(
    amount_in: u64,
    token_in: &str,
    token_out: &str,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let url = format!(
        "https://lite-api.jup.ag/swap/v1/quote?inputMint={}&outputMint={}&amount={}&slippageBps=50",
        token_in, token_out, amount_in
    );

    let mut delay_ms = 500u64;

    for attempt in 0..5 {
        sleep(Duration::from_millis(delay_ms)).await;

        let response = reqwest::get(&url).await?;

        if response.status() == 429 {
            // Rate limited — back off and retry
            delay_ms *= 2;
            continue;
        }

        if !response.status().is_success() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Jupiter API error: {}", response.status()),
            )));
        }

        let quote: JupiterQuoteResponse = response.json().await?;
        return Ok((quote.out_amount, quote.other_amount_threshold));
    }

    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Jupiter API: max retries exceeded (429)",
    )))
}
