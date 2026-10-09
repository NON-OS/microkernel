//! Swaps between ETH, NOX and USDC from the public account, on Uniswap V2 on
//! mainnet, worked out in full before anything is signed.

mod build;
pub mod calldata;
mod checks;
#[cfg(feature = "fuzzing")]
pub(crate) mod fuzz_hook;
pub mod math;
mod order;
mod parse;
mod parse_nox;
pub mod quote;
mod reads;
mod review;
pub mod route;
mod venue;

pub use order::{Swap, SwapChecked, SwapReview};
pub use review::review_swap;
pub use venue::{Pair, Venue, MAINNET};
