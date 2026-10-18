//! A send, worked out in full before anything is signed.
//!
//! One batch over Tor reads the balance, nonce, fees, recipient code and a gas
//! estimate, and for NOX the token's rules and a simulated transfer. The answer
//! is a refusal in a sentence, or a transaction with what leaves, arrives and costs.

#[path = "review_answers.rs"]
mod answers;
#[path = "review_send.rs"]
mod send;

pub use send::review;
