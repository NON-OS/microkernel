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

//! The protocol's start on an open channel, as Linux's
//! storvsc_channel_init: BEGIN_INITIALIZATION, a protocol version the host
//! takes (newest first), QUERY_PROPERTIES for the largest transfer, then
//! END_INITIALIZATION. One channel: no sub-channels are asked for.

use nonos_vmbus::{Host, Shared};

use crate::constants::layout::DATA_BYTES;
use crate::constants::timing::INIT_MS;
use crate::constants::vstor::{
    INIT_ID, OP_BEGIN_INIT, OP_END_INIT, OP_QUERY_PROPERTIES, PACKET, VERSIONS,
};
use crate::hba::{VmHba, IN_MAX};
use crate::vstor::{self, Packet};

/// The smallest transfer worth serving: one 4 KiB block.
const MIN_TRANSFER: u32 = 4096;

fn ask<H: Host, M: Shared>(hba: &mut VmHba<H, M>, p: &Packet) -> Result<[u8; PACKET], &'static str> {
    let mut out = [0u8; IN_MAX];
    let n = hba.transact(INIT_ID, p, 0, INIT_MS, &mut out).map_err(|_| "host silent")?;
    let mut a = [0u8; PACKET];
    a[..n.min(PACKET)].copy_from_slice(&out[..n.min(PACKET)]);
    if n < PACKET {
        return Err("answer short");
    }
    Ok(a)
}

/// The version agreed.
pub fn run<H: Host, M: Shared>(hba: &mut VmHba<H, M>) -> Result<u16, &'static str> {
    if !vstor::done(&ask(hba, &vstor::op(OP_BEGIN_INIT))?) {
        return Err("begin refused");
    }
    let mut agreed = None;
    for v in VERSIONS {
        if vstor::done(&ask(hba, &vstor::version(v))?) {
            agreed = Some(v);
            break;
        }
    }
    let version = agreed.ok_or("no protocol version in common")?;
    let props = ask(hba, &vstor::op(OP_QUERY_PROPERTIES))?;
    let max = vstor::max_transfer(&props).ok_or("properties refused")?;
    if max < MIN_TRANSFER {
        return Err("transfers too small");
    }
    // Whole pages: every page of a transfer starts at its first byte.
    hba.limit = (max as usize).min(DATA_BYTES) & !0xFFF;
    if !vstor::done(&ask(hba, &vstor::op(OP_END_INIT))?) {
        return Err("end refused");
    }
    Ok(version)
}
