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

//! The datagrams of one received NTB (USB NCM 1.0, section 3): its header
//! (nth.rs), then each NDP along the next-index chain (ndp.rs). Every read
//! is inside the block. An NDP is read at most once: a chain that comes
//! back to one already read ends there, so the walk ends within the
//! block's own length and no datagram is taken twice.

use alloc::collections::VecDeque;

use nonos_usbnet::xhci::BULK_MAX;

use super::ndp::ndp;
use super::nth::block;

/// The most datagrams queued from one block: this capsule's bound on what
/// waits in `pending`, not a number the specification gives.
pub const MAX_DATAGRAMS: usize = 40;

/// Queue each datagram of the NTB16, or the NTB32 when `ntb32`, in `raw`
/// as (index, length).
pub fn datagrams(raw: &[u8], rx_max: usize, ntb32: bool, out: &mut VecDeque<(usize, usize)>) {
    let Some((f, b, mut at)) = block(raw, rx_max, ntb32) else { return };
    // One bit per byte a block of one bulk transfer can hold.
    let mut read = [0u64; BULK_MAX / 64];
    while at != 0 && at >= f.nth_len {
        let Some(word) = read.get_mut(at / 64) else { return };
        if *word & (1 << (at % 64)) != 0 {
            return;
        }
        *word |= 1 << (at % 64);
        match ndp(f, b, at, MAX_DATAGRAMS, out) {
            Some(next) => at = next,
            None => return,
        }
    }
}
