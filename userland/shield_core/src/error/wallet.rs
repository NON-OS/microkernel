//! What the shells see. Each layer's failure arrives unchanged, so a screen names the stage
//! that refused and no message is built from a secret. `lift!` joins the layers for `?`.

use super::{CustodyError, NetError, ProveError, StoreError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalletError {
    Custody {
        source: CustodyError,
    },
    Store {
        source: StoreError,
    },
    Net {
        source: NetError,
    },
    Prove {
        source: ProveError,
    },
    /// The balance cannot cover the amount plus the fee.
    Insufficient,
    NoSpendPlan,
    Address,
    Amount,
    /// The confirmed send has no current review: never made, already sent, or expired.
    ReviewExpired,
    /// No account has that number, or the wallet already holds the most it can.
    NoSuchAccount,
    Unavailable,
    /// A private amount that is not a standard size: 1, 2 or 5 followed by zeros.
    NotStandard,
    FeeTooHigh,
    /// The notes that would pay landed too recently to move without the owner going early.
    TooSoon,
    /// The fee moved after it was quoted. Nothing was proved, and a new quote shows the fee now.
    FeeChanged,
}

impl core::error::Error for WalletError {}

macro_rules! lift {
    ($from:ty, $variant:ident) => {
        impl From<$from> for WalletError {
            fn from(source: $from) -> WalletError {
                WalletError::$variant { source }
            }
        }
    };
}

lift!(CustodyError, Custody);
lift!(StoreError, Store);
lift!(NetError, Net);
lift!(ProveError, Prove);
