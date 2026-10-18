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

//! Who may speak the link protocol to a Wi-Fi driver. The link carries raw
//! Ethernet frames onto the air, below the stack and the route the person
//! chose, so a driver serves it only to the process that owns net.core:
//! anyone else could send frames around Nym and Anyone. The settings panel
//! and the setup wizard reach the driver too (the kernel's held table), but
//! for the Wi-Fi control family alone: scan, join, status. The driver finds
//! net.core's owner in the registry and checks the sender of each request,
//! as mk_ipc_recv_from reports it.

use super::wire::{write_response_header, HDR_LEN, MAGIC_NNET};

/// The status a link request from anyone else is refused with: EPERM.
pub const STATUS_NOT_PERMITTED: i32 = -1;

/// Whether `req` is a link-protocol frame, whatever its op.
pub fn is_link(req: &[u8]) -> bool {
    req.get(..4) == Some(&MAGIC_NNET.to_le_bytes()[..])
}

/// Whether `sender` may be served the link: it is net.core's owner, as the
/// registry names it now. No owner, or a sender of 0 (the kernel), never is.
pub fn link_admits(sender: u32, net_core: Option<u32>) -> bool {
    sender != 0 && net_core == Some(sender)
}

/// The refusal for a link request, in the link's own shape: its op and
/// request id back, and STATUS_NOT_PERMITTED, so the caller is answered
/// rather than left waiting. 0 when `out` cannot hold it.
pub fn refuse_link(req: &[u8], out: &mut [u8]) -> usize {
    let (op, request_id) = match req.first_chunk::<HDR_LEN>() {
        Some(h) => {
            (u16::from_le_bytes([h[6], h[7]]), u32::from_le_bytes([h[12], h[13], h[14], h[15]]))
        }
        None => (0, 0),
    };
    let Some(status) = out.get_mut(HDR_LEN..HDR_LEN + 4) else { return 0 };
    status.copy_from_slice(&STATUS_NOT_PERMITTED.to_le_bytes());
    write_response_header(out, op, request_id, 4).map_or(0, |n| n + 4)
}
