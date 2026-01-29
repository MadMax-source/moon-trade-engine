use reqwest::Client;
use serde::Serialize;
use std::time::Duration;
use crate::constants::WSOL_MINT;
use crate::execution::jupiter as jupiter_client;
use crate::execution::jupiter_types::JupiterQuoteResponse;
use crate::execution::errors::SwapError;
use crate::execution::priority::PriorityLevel;
use solana_sdk::signature::Keypair;

pub async fn get_buy_quote(
    input_mint: &str,
    amount: u64,
    slippage_bps: u16,
) -> Result<JupiterQuoteResponse, SwapError> {
    jupiter_client::get_quote(input_mint, WSOL_MINT, amount, slippage_bps).await
}

pub async fn build_buy_swap_tx(
    quote: &JupiterQuoteResponse,
    user_pubkey: &str,
) -> Result<String, SwapError> {
    jupiter_client::build_swap_tx(quote, user_pubkey, PriorityLevel::High).await
}

pub async fn sign_and_send_tx(
    rpc_url: &str,
    base64_tx: &str,
    keypair: &Keypair,
) -> Result<String, SwapError> {
    jupiter_client::sign_and_send_tx(rpc_url, base64_tx, keypair).await
}

