use dotenvy::dotenv;

mod hand;
mod pointer;
mod config;
mod price;
mod constants;
mod execution;

use std::error::Error;
use hand::{HandManager, LockRules};
use pointer::pointer::{Pointer, PointerSignal};
use config::strategy::*;
use price::feed::fetch_sol_price_usd;
use execution::{buy, sell};
use solana_sdk::signature::Signer;

use solana_sdk::signature::Keypair;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    println!(
        "JUP_API_KEY loaded: {}",
        env::var("JUP_API_KEY").is_ok()
    );

    println!("Fixed triggers:");
    println!("  BUY_TRIGGER_USD = ${}", BUY_TRIGGER_USD);
    println!("  SELL_TRIGGER_USD = ${}", SELL_TRIGGER_USD);
    println!("  BUY_SIZE_PCT = {:.2}%", BUY_SIZE_PCT * 100.0);
    println!("----------------------------------------");

    let mut pointer = Pointer::new();
    let mut hand_manager = HandManager::new();

    let rpc_url = env::var("RPC_URL")?;
    let keypair = Keypair::from_base58_string(&env::var("WALLET_PRIVATE_KEY")?);
    let pubkey = keypair.pubkey().to_string();

    loop {
        let sol_price = fetch_sol_price_usd().await?;

        let buy_size_usd = sol_price * BUY_SIZE_PCT;
        let buy_size_sol = buy_size_usd / sol_price;

        println!("Price tick: ${:.6}", sol_price);
        println!(
            "  Buy size: {:.2} USD → {:.6} SOL",
            buy_size_usd, buy_size_sol
        );

        // =====================================================
        // ✅ ALWAYS CHECK SELL CONDITIONS (ADDED BLOCK)
        // =====================================================
        let sell_hands = LockRules::unlock_batch(&mut hand_manager.hands, sol_price);

        for hand in sell_hands {
            let lamports = (hand.size_sol * 1_000_000_000.0) as u64;

            println!(
                "→ Selling {:.6} SOL from hand @ ${:.6}",
                hand.size_sol, hand.price
            );

            let quote =
                sell::get_sell_quote(constants::USDC_MINT, lamports, 50).await?;
            let tx = sell::build_sell_swap_tx(&quote, &pubkey).await?;
            let sig = sell::sign_and_send_tx(&rpc_url, &tx, &keypair)?;

            println!("✅ SELL executed | tx: {}", sig);
        }
        // =====================================================

        if let Some(signal) = pointer.update(sol_price) {
            match signal {
                PointerSignal::BuyStep => {
                    println!("📉 BUY STEP triggered");

                    let buy_size_usd = BUY_SIZE_PCT * sol_price;
                    let usdc_amount = (buy_size_usd * 1_000_000.0) as u64;

                    println!("→ Requesting Jupiter BUY quote (USDC input)...");
                    let quote =
                        buy::get_buy_quote(constants::USDC_MINT, usdc_amount, 50)
                            .await?;

                    println!("→ Building BUY transaction...");
                    let tx = buy::build_buy_swap_tx(&quote, &pubkey).await?;

                    println!("→ Sending BUY transaction...");
                    let sig =
                        buy::sign_and_send_tx(&rpc_url, &tx, &keypair)?;

                    println!("✅ BUY executed | tx: {}", sig);

                    hand_manager.open_hand(sol_price, buy_size_sol);
                }

                PointerSignal::SellStep => {
                    // LEFT INTACT (you asked not to change it)
                    println!("📈 SELL STEP triggered");
                }
            }
        }

        hand_manager.print_hands();
        println!("----------------------------------------");

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
}







/*
use dotenvy::dotenv;

mod hand;
mod pointer;
mod config;
mod price;
mod constants;
mod execution;

use std::error::Error;
use hand::{HandManager, LockRules};
use pointer::pointer::{Pointer, PointerSignal};
use config::strategy::*;
use price::feed::fetch_sol_price_usd;
use execution::{buy, sell};
use solana_sdk::signature::Signer; // needed for keypair.pubkey()

use solana_sdk::signature::Keypair;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    println!(
        "JUP_API_KEY loaded: {}",
        env::var("JUP_API_KEY").is_ok()
    );

    println!("Fixed triggers:");
    println!("  BUY_TRIGGER_USD = ${}", BUY_TRIGGER_USD);
    println!("  SELL_TRIGGER_USD = ${}", SELL_TRIGGER_USD);
    println!("  BUY_SIZE_PCT = {:.2}%", BUY_SIZE_PCT * 100.0);
    println!("----------------------------------------");
    let mut pointer = Pointer::new();
    let mut hand_manager = HandManager::new();
    let rpc_url = env::var("RPC_URL")?;
    let keypair = Keypair::from_base58_string(&env::var("WALLET_PRIVATE_KEY")?);
    let pubkey = keypair.pubkey().to_string();

    loop {
        let sol_price = fetch_sol_price_usd().await?;

        let buy_size_usd = sol_price * BUY_SIZE_PCT;
        let buy_size_sol = buy_size_usd / sol_price;

        println!("Price tick: ${:.6}", sol_price);
        println!(
            "  Buy size: {:.2} USD → {:.6} SOL",
            buy_size_usd, buy_size_sol
        );

        if let Some(signal) = pointer.update(sol_price) {
            match signal {
                PointerSignal::BuyStep => {
                    println!("📉 BUY STEP triggered");

                    // Corrected for USDC
let buy_size_usd = BUY_SIZE_PCT * sol_price;        // USD you want to spend
let usdc_amount = (buy_size_usd * 1_000_000.0) as u64; // USDC has 6 decimals

println!("→ Requesting Jupiter BUY quote (USDC input)...");
let quote = buy::get_buy_quote(constants::USDC_MINT, usdc_amount, 50).await?;


                    println!("→ Building BUY transaction...");
                    let tx = buy::build_buy_swap_tx(&quote, &pubkey).await?;

                    println!("→ Sending BUY transaction...");
                    let sig = buy::sign_and_send_tx(&rpc_url, &tx, &keypair)?;

                    println!("✅ BUY executed | tx: {}", sig);

                    hand_manager.open_hand(sol_price, buy_size_sol);
                }

PointerSignal::SellStep => {
    println!("📈 SELL STEP triggered");

    let sell_hands = LockRules::unlock_batch(&mut hand_manager.hands, sol_price);

    for hand in sell_hands {
        let lamports = (hand.size_sol * 1_000_000_000.0) as u64;

        println!(
            "→ Selling {:.6} SOL from hand @ ${:.6}",
            hand.size_sol, hand.price
        );

        let quote = sell::get_sell_quote(constants::WSOL_MINT, lamports, 50).await?;
        let tx = sell::build_sell_swap_tx(&quote, &pubkey).await?;
        let sig = sell::sign_and_send_tx(&rpc_url, &tx, &keypair)?;

        println!("✅ SELL executed | tx: {}", sig);
    }
}


            }
        }

        hand_manager.print_hands();
        println!("----------------------------------------");

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
}
*/


/*

use dotenvy::dotenv;
mod hand;
mod pointer;
mod config;
mod price;
mod constants;

use std::error::Error;
use hand::{HandManager, LockRules};
use pointer::pointer::{Pointer, PointerSignal};
use config::strategy::*;
use price::feed::fetch_sol_price_usd;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    println!(
        "JUP_API_KEY loaded: {}",
        std::env::var("JUP_API_KEY").is_ok()
    );

    println!("Fixed triggers:");
    println!("  BUY_TRIGGER_USD = ${}", BUY_TRIGGER_USD);
    println!("  SELL_TRIGGER_USD = ${}", SELL_TRIGGER_USD);
    println!("  BUY_SIZE_PCT = {:.2}%", BUY_SIZE_PCT * 100.0);
    println!("----------------------------------------");

    let mut pointer = Pointer::new();
    let mut hand_manager = HandManager::new();

    loop {
        let sol_price = fetch_sol_price_usd().await?;
        let buy_size_usd = sol_price * BUY_SIZE_PCT;
        let buy_size_sol = buy_size_usd / sol_price;

        println!("Price tick: ${:.6}", sol_price);
        println!(
            "  Buy size calculation: {:.2} USD → {:.6} SOL (13.33%)",
            buy_size_usd, buy_size_sol
        );

        if let Some(signal) = pointer.update(sol_price) {
            match signal {
                PointerSignal::BuyStep => {
                    println!("📉 BUY STEP detected ($0.50 drop from last reference)");
                    println!("  Pointer reference updated → ${:.6}", sol_price);

                    // Open a new hand
                    hand_manager.open_hand(sol_price, buy_size_sol);
                }
                PointerSignal::SellStep => {
                    println!("📈 SELL STEP detected ($0.43 rise from last reference)");
                    println!("  Pointer reference updated → ${:.6}", sol_price);

                    LockRules::unlock_batch(&mut hand_manager.hands, sol_price);
                }
            }
        }

        hand_manager.print_hands();

        println!("----------------------------------------");
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
}


*/




