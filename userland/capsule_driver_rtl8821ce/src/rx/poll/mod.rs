// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Take the next received 802.11 frame off the RX ring. The hardware write index
//! says how many slots are ready; each is parsed, and firmware-command, error
//! and oversized frames are skipped (their slots re-armed) until a deliverable
//! frame is found. Its body is copied out, the slot is re-armed, the host read
//! index advances and is written back to hand the slot to the card. This is the
//! read half of rtw88 `rtw_pci_rx_napi`, shaped as a single-frame poll so it
//! backs the `poll_rx` half of the net_core link contract. Checked against a
//! modeled device and DMA memory in `rtl8821ce_proofs`.
//!
//! Everything the device wrote is untrusted: the descriptor's length, driver
//! info size and shift are only used once the frame they describe is shown to
//! lie inside its own slot and inside the mapped buffer, and a slot whose
//! buffer the mapping does not cover is skipped rather than read.

mod info;
#[cfg(test)]
mod length;

pub use info::poll_one_info;
#[cfg(test)]
pub use length::poll_one;
