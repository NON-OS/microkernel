//! Walking the log back into a state, each row opened under the counter it carries.
//! A row that does not authenticate stops the walk. Skipping it would hide a tamper as a
//! shorter history.

use super::frame::decode as unframe;
use super::key::AAD;
use super::row::decode as read_row;
use super::state::StoreState;
use crate::error::StoreError;
use nonos_seal::{SealState, TAG_LEN};

/// Walk the file, stopping at the first row that does not authenticate.
pub(super) fn replay(bytes: &[u8], key: [u8; 32]) -> Result<(StoreState, u64), StoreError> {
    let reader = SealState::new(key, 0);
    let mut state = StoreState::empty();
    let mut next = 0u64;
    let mut at = 0usize;
    while at < bytes.len() {
        let tail = bytes.get(at..).ok_or(StoreError::RowShape)?;
        let (counter, sealed, used) = unframe(tail)?;
        let mut plain = vec![0u8; sealed.len().saturating_sub(TAG_LEN)];
        match reader.open_at(counter, AAD, sealed, &mut plain) {
            Ok(n) if n == plain.len() => state.apply(read_row(&plain)?),
            Ok(_) => return Err(StoreError::RowShape),
            Err(_) => return Err(StoreError::RowAuth),
        }
        next = counter.checked_add(1).ok_or(StoreError::NonceExhausted)?;
        at = at.checked_add(used).ok_or(StoreError::RowShape)?;
    }
    Ok((state, next))
}
