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

//! The served namespace's layout as the client maps sectors onto it: the
//! LBA size and capacity from the capsule's identify-namespace and capacity
//! replies, and the per-command LBA ceiling from the controller's MDTS. Asked
//! once per capsule generation: a respawned capsule may serve another
//! controller, so a new generation asks again.

use spin::Mutex;

use super::super::error::DriverNvmeError;
use super::super::protocol::{encode_request, MAX_RW_PAYLOAD_BYTES, OP_CAPACITY};
use super::super::state;
use super::identify_controller::identify_controller;
use super::identify_namespace::identify_namespace;
use super::lba_map::{addressable, lbas_per_request};
use super::read::u64_at;
use super::seq::next_request_id;
use super::status_map::lift;
use super::transport::round_trip;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Layout {
    pub lba_size: u32,
    /// LBAs one capsule read or write may carry.
    pub per_request: u64,
    pub capacity_lbas: u64,
}

static CACHE: Mutex<Option<(u64, Layout)>> = Mutex::new(None);

/// The layout, asked of the capsule when this generation has not been.
/// `Unsupported` when the namespace has no I/O path or an LBA size the
/// client cannot address.
pub(super) fn layout() -> Result<Layout, DriverNvmeError> {
    let generation = state::shared_state().generation();
    if let Some((cached_for, layout)) = *CACHE.lock() {
        if cached_for == generation {
            return Ok(layout);
        }
    }
    let ns = identify_namespace()?;
    if !addressable(ns.lba_size, MAX_RW_PAYLOAD_BYTES) {
        crate::log::warn!(
            "[NVME] namespace {} uses {}-byte LBAs, which the client cannot address",
            ns.nsid,
            ns.lba_size
        );
        return Err(DriverNvmeError::Unsupported);
    }
    let mdts = identify_controller()?.mdts;
    let per_request = lbas_per_request(mdts, ns.lba_size, MAX_RW_PAYLOAD_BYTES as u64);
    if per_request == 0 {
        return Err(DriverNvmeError::Unsupported);
    }
    let layout = Layout { lba_size: ns.lba_size, per_request, capacity_lbas: capacity_lbas()? };
    *CACHE.lock() = Some((generation, layout));
    Ok(layout)
}

/// The capsule's capacity reply: the namespace size in its own LBAs.
fn capacity_lbas() -> Result<u64, DriverNvmeError> {
    let request_id = next_request_id();
    let frame = encode_request(OP_CAPACITY, 0, request_id, &[]);
    let resp = round_trip(request_id, frame)?;
    if resp.status != 0 {
        return Err(lift(resp.status));
    }
    if resp.body.len() < 8 {
        return Err(DriverNvmeError::ProtocolMismatch);
    }
    Ok(u64_at(&resp.body, 0))
}
