use reqwest::Client;
use serde::Serialize;
use std::time::Duration;

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
    transaction::VersionedTransaction,
};

use crate::constants::{WSOL_MINT, MAX_COMPUTE_LAMPORTS};
use crate::execution::priority::PriorityLevel as AppPriorityLevel;
use crate::execution::errors::SwapError;
use crate::execution::jupiter_types::{JupiterQuoteResponse, JupiterSwapResponse};

use base64;
use bincode;

const JUP_QUOTE_URL: &str = "https://lite-api.jup.ag/swap/v1/quote";
const JUP_SWAP_URL: &str = "https://lite-api.jup.ag/swap/v1/swap";


#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SwapRequest<'a> {
    quote_response: &'a JupiterQuoteResponse,
    user_public_key: String,
    dynamic_compute_unit_limit: bool,
    dynamic_slippage: bool,
    prioritization_fee_lamports: PriorityFee,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PriorityFee {
    priority_level_with_max_lamports: JupiterPriorityConfig,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct JupiterPriorityConfig {
    max_lamports: u64,
    priority_level: String,
}

pub async fn get_sell_quote(
    output_mint: &str,
    amount: u64,
    slippage_bps: u16,
) -> Result<JupiterQuoteResponse, SwapError> {
    if amount == 0 {
        return Err(SwapError::InvalidAmount);
    }

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|_| SwapError::NetworkTimeout)?;

    let url = format!(
        "{}?inputMint={}&outputMint={}&amount={}&slippageBps={}",
        JUP_QUOTE_URL,
        WSOL_MINT,
        output_mint,
        amount,
        slippage_bps
    );

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|_| SwapError::NetworkTimeout)?;

    if !res.status().is_success() {
        let err = res.text().await.unwrap_or_default();
        return Err(SwapError::JupiterApi(err));
    }

    res.json::<JupiterQuoteResponse>()
        .await
        .map_err(|_| SwapError::Serialization)
}



pub async fn build_sell_swap_tx(
    quote: &JupiterQuoteResponse,
    user_pubkey: &str,
) -> Result<String, SwapError> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|_| SwapError::NetworkTimeout)?;

    let priority = AppPriorityLevel::Medium;

    let body = SwapRequest {
        quote_response: quote,
        user_public_key: user_pubkey.to_string(),
        dynamic_compute_unit_limit: true,
        dynamic_slippage: true,
        prioritization_fee_lamports: PriorityFee {
            priority_level_with_max_lamports: JupiterPriorityConfig {
                max_lamports: MAX_COMPUTE_LAMPORTS,
                priority_level: priority.as_str().to_string(),
            },
        },
    };

    let res = client
        .post(JUP_SWAP_URL)
        .json(&body)
        .send()
        .await
        .map_err(|_| SwapError::NetworkTimeout)?;

    if !res.status().is_success() {
        let err = res.text().await.unwrap_or_default();
        return Err(SwapError::JupiterApi(err));
    }

    let swap = res
        .json::<JupiterSwapResponse>()
        .await
        .map_err(|_| SwapError::Serialization)?;

    Ok(swap.swap_transaction)
}


pub async fn sign_and_send_tx(
    rpc_url: &str,
    base64_tx: &str,
    keypair: &Keypair,
) -> Result<String, SwapError> {
    let rpc = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let tx_bytes = base64::decode(base64_tx).map_err(|_| SwapError::Serialization)?;

    let tx: VersionedTransaction =
        bincode::deserialize(&tx_bytes).map_err(|_| SwapError::Serialization)?;

    let signed_tx =
        VersionedTransaction::try_new(tx.message, &[keypair]).map_err(|_| SwapError::Signing)?;

    let sig = rpc
        .send_and_confirm_transaction(&signed_tx)
        .await
        .map_err(|e| SwapError::Rpc(e.to_string()))?;

    Ok(sig.to_string())
}





/*

use reqwest::Client;
use serde::Serialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
    transaction::VersionedTransaction,
};
use crate::constants::WSOL_MINT;
use base64;
use bincode;

const JUP_QUOTE_URL: &str = "https://lite-api.jup.ag/swap/v1/quote";
const JUP_SWAP_URL: &str = "https://lite-api.jup.ag/swap/v1/swap";

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SwapRequest<'a> {
    quote_response: &'a serde_json::Value,
    user_public_key: String,
    dynamic_compute_unit_limit: bool,
    dynamic_slippage: bool,
    prioritization_fee_lamports: PriorityFee,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PriorityFee {
    priority_level_with_max_lamports: PriorityLevel,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PriorityLevel {
    max_lamports: u64,
    priority_level: String,
}


pub async fn get_sell_quote(
    output_mint: &str, // ← USDC
    amount: u64,
    slippage_bps: u16,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let client = Client::new();

    let url = format!(
        "{}?inputMint={}&outputMint={}&amount={}&slippageBps={}",
        JUP_QUOTE_URL,
        WSOL_MINT,        // selling wSOL
        output_mint,      // receiving USDC
        amount,
        slippage_bps
    );

    let res = client.get(&url).send().await?;
    if !res.status().is_success() {
        return Err(format!("Quote HTTP {}", res.status()).into());
    }

    Ok(res.json().await?)
}


pub async fn build_sell_swap_tx(
    quote: &serde_json::Value,
    user_pubkey: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let body = SwapRequest {
        quote_response: quote,
        user_public_key: user_pubkey.to_string(),
        dynamic_compute_unit_limit: true,
        dynamic_slippage: true,
        prioritization_fee_lamports: PriorityFee {
            priority_level_with_max_lamports: PriorityLevel {
                max_lamports: 1_000_000,
                priority_level: "veryHigh".to_string(),
            },
        },
    };
    let res = client.post(JUP_SWAP_URL).json(&body).send().await?;
    if !res.status().is_success() {
        let err = res.text().await?;
        return Err(format!("Swap API failed: {}", err).into());
    }
    let json: serde_json::Value = res.json().await?;
    let tx = json
        .get("swapTransaction")
        .and_then(|v| v.as_str())
        .ok_or("swapTransaction missing")?;
    Ok(tx.to_string())
}

pub fn sign_and_send_tx(
    rpc_url: &str,
    base64_tx: &str,
    keypair: &Keypair,
) -> Result<String, Box<dyn std::error::Error>> {
    let rpc = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());
    let tx_bytes = base64::decode(base64_tx)?;
    let tx: VersionedTransaction = bincode::deserialize(&tx_bytes)?;
    let signed_tx = VersionedTransaction::try_new(tx.message, &[keypair])?;
    let sig = rpc.send_and_confirm_transaction(&signed_tx)?;
    Ok(sig.to_string())
}

*/