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

//! Frames out, one per bulk transfer, as Linux v6.12 r8152.c frames them:
//! an 8-byte descriptor, its first dword the frame's length with the
//! first- and last-segment bits (31, 30), its second 0 (no checksum or
//! segmentation offload asked), then the frame, neither padded nor with an
//! FCS, as Linux sends it (so the chip adds both: inferred, unverified on
//! hardware). A stalled pipe is brought back before its error
//! goes up, so the next transfer finds it working.

use nonos_usbnet::halt::clear_halt;
use nonos_usbnet::nic::{ETH_FRAME_MAX, ETH_HEADER};
use nonos_usbnet::xhci::{E_INVAL, E_IO, E_PIPE};
use nonos_usbnet::Bus;

use super::state::Rtl;

pub const TX_DESC: usize = 8;
const FIRST: u32 = 1 << 31;
const LAST: u32 = 1 << 30;

/// The transfer for `frame` in `out`, its length; `None` for a frame of
/// other than 14 to 1514 bytes, or one `out` cannot hold.
pub fn encode(frame: &[u8], out: &mut [u8]) -> Option<usize> {
    let n = TX_DESC + frame.len();
    if !(ETH_HEADER..=ETH_FRAME_MAX).contains(&frame.len()) || out.len() < n {
        return None;
    }
    out[..4].copy_from_slice(&(frame.len() as u32 | FIRST | LAST).to_le_bytes());
    out[4..TX_DESC].fill(0);
    out[TX_DESC..n].copy_from_slice(frame);
    Some(n)
}

pub(super) fn send<B: Bus>(r: &mut Rtl<B>, frame: &[u8]) -> Result<(), i32> {
    let Some(n) = encode(frame, &mut r.tx) else { return Err(E_INVAL) };
    match r.bus.bulk_out(&r.tx[..n]) {
        Ok(m) if m == n => Ok(()),
        Ok(_) => Err(E_IO),
        Err(e) => Err(recover(&mut r.bus, e, r.pipes.bulk_out)),
    }
}

pub(super) fn recover<B: Bus>(bus: &mut B, e: i32, endpoint: u8) -> i32 {
    if e == E_PIPE {
        let _ = clear_halt(bus, endpoint);
    }
    e
}
