//! The history of the active account, as a screen lists it, folded from the rows its store
//! keeps (`store::activity`), so it is the same after a restart as before it.

use super::Wallet;
use crate::error::WalletError;
use crate::ffi::evm::to_hex;
use crate::ffi::format_amount;
use crate::net::pool::ACTIVE;
use crate::store::activity::{history, Stage, DEPOSIT_SENT, RECEIVED, SENT, WITHDRAWN};

/// One entry, oldest first.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HistoryItem {
    /// deposit, sent, withdrawn or received.
    pub kind: String,
    /// ETH or NOX, or empty for an asset this build does not know.
    pub coin: String,
    pub amount: String,
    /// Seconds since 1970 when it was recorded.
    pub at: u64,
    /// on its way, settling, done or taken back.
    pub stage: String,
    pub tx: Option<String>,
}

impl Wallet {
    /// The active account's history, oldest first.
    pub fn history(&self) -> Result<Vec<HistoryItem>, WalletError> {
        self.with(|s| history(s.state().activity()).iter().map(item).collect())
    }
}

fn item(e: &crate::store::activity::Entry) -> HistoryItem {
    let coin = ACTIVE.assets.iter().find(|a| a.id == e.asset_id).map(|a| a.coin);
    let kind = match e.kind {
        DEPOSIT_SENT => "deposit",
        SENT => "sent",
        WITHDRAWN => "withdrawn",
        RECEIVED => "received",
        _ => "other",
    };
    let stage = match e.stage {
        Stage::OnItsWay => "on its way",
        Stage::Settling => "settling",
        Stage::Done => "done",
        Stage::TakenBack => "taken back",
    };
    HistoryItem {
        kind: kind.into(),
        coin: coin.map(|c| format!("{c:?}").to_uppercase()).unwrap_or_default(),
        amount: coin.map(|c| format_amount(c, e.value)).unwrap_or_else(|| e.value.to_string()),
        at: e.at,
        stage: stage.into(),
        tx: e.tx.as_ref().map(|t| to_hex(t)),
    }
}
