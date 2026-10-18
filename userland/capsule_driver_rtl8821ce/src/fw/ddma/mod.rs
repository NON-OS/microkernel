// NONOS Operating System (AGPL-3.0-or-later)

//! The direct-DMA (DDMA) channel that copies a firmware chunk from the card's
//! packet buffer into on-chip 8051 memory during firmware download. Each
//! transfer programs the source, destination and a control word (length, a
//! running checksum, and the channel-owned bit that clears when the copy
//! finishes), then waits for completion. The engine accumulates a checksum
//! across the chunks of a section and reports whether it validated. These are
//! the rtw88 8821c register semantics, reimplemented; the exact register
//! program is checked against a modeled device in `rtl8821ce_proofs`.

mod regs;
mod section;
mod transfer;

pub use regs::*;
pub use section::record_section;
pub use transfer::{enable_download, reset_checksum, transfer};
