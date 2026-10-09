//! Appending a row: sealed under the next nonce, framed, and synced before returning.
//! A nonce counter that would wrap is refused, since a repeated nonce breaks the seal.

use super::frame::{encode as frame, MAX_ROW};
use super::key::AAD;
use super::log::NoteLog;
use super::row::Row;
use crate::error::StoreError;
use nonos_seal::{SealError, TAG_LEN};
use std::io::Write;

impl NoteLog {
    /// Seal a row and append it. The caller updates its state only after this succeeds.
    pub fn append(&mut self, row: &Row) -> Result<(), StoreError> {
        let payload = row.encode();
        if payload.len().saturating_add(TAG_LEN) > MAX_ROW {
            return Err(StoreError::RowShape);
        }
        let mut sealed = vec![0u8; payload.len().saturating_add(TAG_LEN)];
        let (counter, n) =
            self.seal.seal_next(AAD, &payload, &mut sealed).map_err(|e| match e {
                SealError::NonceExhausted => StoreError::NonceExhausted,
                _ => StoreError::RowShape,
            })?;
        sealed.truncate(n);
        let bytes = frame(counter, &sealed)?;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|_| StoreError::Io)?;
        file.write_all(&bytes).map_err(|_| StoreError::Io)?;
        file.sync_data().map_err(|_| StoreError::Io)
    }
}
