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
//! Read a scanout's raw EDID from the device, for the wire's panel (its
//! modes and physical size). Every byte here crosses the trust boundary, so
//! the reply's type, the declared length, the length the queue reported and
//! the base block's magic are checked before any of it is handed on; what
//! is handed on is then parsed by nonos_edid, which trusts none of it.

use super::hdr::{Hdr, HDR_LEN, RESP_HDR_LEN};
use super::le::le_u32;
use crate::constants::{VG_CMD_GET_EDID, VG_RESP_OK_EDID};
use crate::device::virtqueue::ControlQueue;

const REQ_LEN: usize = HDR_LEN + 8; // __le32 scanout, __le32 padding
const BODY_OFFSET: usize = RESP_HDR_LEN + 8; // __le32 size, __le32 padding
pub const EDID_MAX_LEN: usize = 1024;
const EDID_BLOCK_LEN: usize = 128;
const RESP_LEN: usize = BODY_OFFSET + EDID_MAX_LEN;
const EDID_MAGIC: [u8; 8] = [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00];

/// The blob's length, its bytes in `out`.
pub fn get_edid(
    q: &ControlQueue,
    scanout: u32,
    out: &mut [u8; EDID_MAX_LEN],
) -> Result<usize, &'static str> {
    let mut req = [0u8; REQ_LEN];
    Hdr::new(VG_CMD_GET_EDID, 0).write(&mut req);
    req[HDR_LEN..HDR_LEN + 4].copy_from_slice(&scanout.to_le_bytes());
    let used = q.submit(&req, RESP_LEN as u32)?.used_len as usize;
    if used < BODY_OFFSET {
        return Err("virtio-gpu: short edid response");
    }
    let mut head = [0u8; BODY_OFFSET];
    q.read_response(REQ_LEN, &mut head);
    let hdr = Hdr::parse(&head[..HDR_LEN]).ok_or("virtio-gpu: bad edid header")?;
    if hdr.type_ != VG_RESP_OK_EDID {
        return Err("virtio-gpu: edid rejected");
    }
    let size = le_u32(&head, RESP_HDR_LEN).ok_or("virtio-gpu: bad edid body")? as usize;
    if !(EDID_BLOCK_LEN..=EDID_MAX_LEN).contains(&size) || size > used - BODY_OFFSET {
        return Err("virtio-gpu: edid size out of range");
    }
    let mut all = [0u8; RESP_LEN];
    q.read_response(REQ_LEN, &mut all[..BODY_OFFSET + size]);
    out[..size].copy_from_slice(&all[BODY_OFFSET..BODY_OFFSET + size]);
    if out[..EDID_MAGIC.len()] != EDID_MAGIC {
        return Err("virtio-gpu: edid magic mismatch");
    }
    Ok(size)
}
