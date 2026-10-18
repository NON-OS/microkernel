//! Recording what one fetched history holds for a view-only account.

use super::leaves;
use crate::error::WalletError;
use crate::notes::NoteStatus;
use crate::store::Row;
use crate::wallet::watch::Watched;
use crate::wallet::{scan_history_as, History};

/// Record what the history holds for one view-only account. It has no deposits of its own.
pub(in crate::ffi::wallet) fn sync_watched(
    w: &mut Watched,
    history: &History,
) -> Result<(), WalletError> {
    let held = w.state.held();
    let scan = scan_history_as(history, &w.viewer(), &[], &held);
    let known: Vec<[u64; 4]> = held.iter().map(|n| n.cm).collect();
    let rows: Vec<Row> = scan
        .received
        .into_iter()
        .filter(|n| !known.contains(&n.cm))
        .map(Row::Found)
        .chain(scan.spent.into_iter().map(|cm| Row::Status { cm, status: NoteStatus::Spent }))
        .collect();
    for row in rows {
        w.log.append(&row)?;
        w.state.apply(row);
    }
    w.state.mark_pool(&leaves(history));
    Ok(())
}
