//! Row framing: a length, the nonce counter, then the sealed bytes.
//! The counter lets a reload continue the nonce sequence without guessing it. Every length is
//! checked before allocating, so a hostile file cannot force a huge reservation.

use crate::error::StoreError;

/// Row header: the sealed length, then the nonce counter the row was sealed under.
pub const HEADER: usize = 4 + 8;

/// The largest row the reader accepts. Anything past it is a corrupt or hostile file.
pub const MAX_ROW: usize = 4096;

/// Frame a sealed payload for the log.
pub fn encode(counter: u64, sealed: &[u8]) -> Result<Vec<u8>, StoreError> {
    let len = u32::try_from(sealed.len()).map_err(|_| StoreError::RowShape)?;
    let mut out = Vec::with_capacity(HEADER.saturating_add(sealed.len()));
    out.extend_from_slice(&len.to_le_bytes());
    out.extend_from_slice(&counter.to_le_bytes());
    out.extend_from_slice(sealed);
    Ok(out)
}

/// Read one frame from `bytes`: counter, sealed slice, bytes consumed. `RowShape` on a bad length.
pub fn decode(bytes: &[u8]) -> Result<(u64, &[u8], usize), StoreError> {
    let shape = || StoreError::RowShape;
    let (len_bytes, rest) = bytes.split_at_checked(4).ok_or_else(shape)?;
    let (counter_bytes, rest) = rest.split_at_checked(8).ok_or_else(shape)?;
    let mut len = [0u8; 4];
    len.copy_from_slice(len_bytes);
    let mut counter = [0u8; 8];
    counter.copy_from_slice(counter_bytes);
    let len = usize::try_from(u32::from_le_bytes(len)).map_err(|_| shape())?;
    if len > MAX_ROW {
        return Err(shape());
    }
    let (sealed, _) = rest.split_at_checked(len).ok_or_else(shape)?;
    let used = HEADER.checked_add(len).ok_or_else(shape)?;
    Ok((u64::from_le_bytes(counter), sealed, used))
}
