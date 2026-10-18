// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The served disk's layout as the client maps sectors onto it. A fixed
//! disk is asked once per capsule generation, since a respawned capsule may
//! serve another device. A removable one is asked before every request: a
//! card swapped in the slot keeps the generation and changes the disk.

use super::decode::decode;
use super::error::WireError;
use super::header::{MAX_TRANSFER_BYTES, OP_GEOMETRY};
use super::lba_map::addressable;
use super::target::Target;
use super::transport::call;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Layout {
    pub lba_bytes: u32,
    pub lba_count: u64,
    /// Blocks one READ or WRITE may carry, at least one.
    pub per_request: u64,
    pub read_only: bool,
}

pub(super) fn layout(t: Target) -> Result<Layout, WireError> {
    let e = t.e;
    let generation = e.state.generation();
    if let Some((asked_of, layout)) = e.slot.layout.lock()[t.at()] {
        if asked_of == generation {
            return Ok(layout);
        }
    }
    let g = decode(&call(t, OP_GEOMETRY, &[])?).ok_or(WireError::ProtocolMismatch)?;
    if !g.media_present() {
        return Err(WireError::NoMedium);
    }
    let buffer = g.max_transfer_bytes.min(MAX_TRANSFER_BYTES);
    if !addressable(g.lba_bytes, buffer) {
        crate::log::warn!(
            "[BLOCK] {} uses {}-byte blocks with {}-byte transfers, which 512-byte sectors cannot map to",
            e.shown,
            g.lba_bytes,
            g.max_transfer_bytes
        );
        return Err(WireError::BlockSize);
    }
    let layout = Layout {
        lba_bytes: g.lba_bytes,
        lba_count: g.lba_count,
        per_request: (buffer / g.lba_bytes) as u64,
        read_only: g.read_only(),
    };
    if !g.removable() {
        e.slot.layout.lock()[t.at()] = Some((generation, layout));
    }
    Ok(layout)
}
