// NONOS Operating System (AGPL-3.0-or-later)

//! The buffer descriptor that hands a slot back to the card.

use super::super::regs::RX_BUF_DESC_SIZE;

/// Build the 8-byte RX buffer descriptor that (re-)arms a slot: it tells the
/// card the slot's size and bus address; the card fills the received length.
pub fn rearm(dma: u64, buf_size: usize) -> [u8; RX_BUF_DESC_SIZE] {
    let mut d = [0u8; RX_BUF_DESC_SIZE];
    d[0..2].copy_from_slice(&(buf_size as u16).to_le_bytes());
    // Bytes 2..4 (total_pkt_size) stay zero; the card writes the length there.
    d[4..8].copy_from_slice(&(dma as u32).to_le_bytes());
    d
}
