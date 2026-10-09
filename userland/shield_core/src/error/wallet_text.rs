//! The sentence each wallet failure is logged and tested by. No sentence is built from a secret.

use super::wallet::WalletError;

impl core::fmt::Display for WalletError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            WalletError::Custody { source } => source.fmt(f),
            WalletError::Store { source } => source.fmt(f),
            WalletError::Net { source } => source.fmt(f),
            WalletError::Prove { source } => source.fmt(f),
            WalletError::Insufficient => f.write_str("the balance does not cover that"),
            WalletError::NoSpendPlan => f.write_str("the held notes cannot form that transfer"),
            WalletError::Address => f.write_str("that recipient address is not valid"),
            WalletError::Amount => f.write_str("that amount is out of range"),
            WalletError::ReviewExpired => f.write_str("that review has expired; review it again"),
            WalletError::NoSuchAccount => f.write_str("there is no such account"),
            WalletError::Unavailable => f.write_str("that is not in this build"),
            WalletError::NotStandard => f.write_str("private amounts come in standard sizes"),
            WalletError::FeeTooHigh => f.write_str("that relay fee is above the pool cap"),
            WalletError::TooSoon => f.write_str("the notes that would pay are too new to move yet"),
            WalletError::FeeChanged => {
                f.write_str("the fee moved since it was shown, so quote it again")
            }
        }
    }
}
