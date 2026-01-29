use thiserror::Error;

#[derive(Error, Debug)]
pub enum SwapError {
    #[error("Invalid amount")]
    InvalidAmount,

    #[error("Quote expired")]
    QuoteExpired,

    #[error("Jupiter API error: {0}")]
    JupiterApi(String),

    #[error("Network timeout")]
    NetworkTimeout,

    #[error("RPC error: {0}")]
    Rpc(String),

    #[error("Serialization error")]
    Serialization,

    #[error("Signing error")]
    Signing,
}
