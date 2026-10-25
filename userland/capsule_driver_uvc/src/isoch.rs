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
//! driver.xhci0's isoch IN ops (B-36): the streaming endpoint configured,
//! then batches of service intervals, each call queueing one batch and
//! collecting the one before. The reply is `cc, packets, 0, 0`, 31 u16
//! length slots, then the packets' bytes packed in order (batch.rs reads
//! it), at most 4034 bytes in all.

use nonos_usbnet::xhci::call;
use nonos_uvc::Alt;

const OP_CONFIGURE_ISOCH: u16 = 0x0018;
const OP_ISOCH_IN: u16 = 0x001A;
/// A reply's data: after its 20-byte header and its status word.
pub const DATA_AT: usize = 24;
/// The most bytes one batch hands back, and the most intervals it asks.
pub const BATCH_BYTES: usize = 4034;
pub const BATCH_MAX: usize = 31;
pub const REPLY_LEN: usize = DATA_AT + 4 + 2 * BATCH_MAX + BATCH_BYTES;

/// The streaming endpoint at `alt`, polled every `interval` (bInterval).
pub fn configure(xhci: u32, slot: u8, alt: &Alt, interval: u8) -> Result<u8, i32> {
    let [m0, m1] = alt.max_packet.to_le_bytes();
    let body = [slot, alt.endpoint, interval, 0, m0, m1, 0, 0];
    let mut resp = [0u8; DATA_AT + 4];
    call(xhci, OP_CONFIGURE_ISOCH, &body, &mut resp)?;
    Ok(resp[DATA_AT])
}

/// Intervals a batch asks for: as many as fit its bytes, one at least.
pub fn per_batch(alt: &Alt) -> u8 {
    let n = BATCH_BYTES / (alt.per_interval as usize).max(1);
    n.clamp(1, BATCH_MAX) as u8
}

/// Queue `n` intervals and collect the batch before into `resp`; the
/// reply's data length.
pub fn isoch_in(xhci: u32, slot: u8, n: u8, resp: &mut [u8]) -> Result<usize, i32> {
    call(xhci, OP_ISOCH_IN, &[slot, 0, n, 0], resp)
}
