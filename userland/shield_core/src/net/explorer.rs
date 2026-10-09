//! Block explorer pages for a sent transaction. The wallet never requests them, and a link
//! opens only when a person chooses, in their own browser.

/// Etherscan's transaction page on mainnet.
pub const MAINNET_TX: &str = "https://etherscan.io/tx/";

/// Etherscan's transaction page on Sepolia.
pub const SEPOLIA_TX: &str = "https://sepolia.etherscan.io/tx/";
