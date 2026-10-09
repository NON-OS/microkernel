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

use super::held::say;
use super::{map::map, Hmb};
use crate::admin::hmb::{enable_dwords, ends_attempt, plan, ENABLE_TIMEOUT_MS};
use crate::admin::set_features::FID_HOST_MEMORY_BUFFER;
use crate::admin::{AdminQueue, ControllerIdentity};
use crate::controller::ControllerInfo;
use crate::error::NvmeResult;
use crate::regs::Regs;

/// Offer the host memory buffer the controller asks for. `Ok(None)` when it
/// asks for none, the memory could not be had, or it refused the buffer;
/// the drive is served without one, as the spec allows. No answer fails
/// the attempt, with the controller disabled before the memory goes.
pub fn host_memory(
    admin: &mut AdminQueue,
    regs: Regs,
    info: ControllerInfo,
    device_id: u64,
    epoch: u64,
    identity: &ControllerIdentity,
) -> NvmeResult<Option<Hmb>> {
    let ask = identity.hmb;
    let Some(p) = plan(ask) else {
        if ask.preferred != 0 {
            say(
                b"host memory buffer not offered: beyond 128 MiB, minimum pages ",
                ask.minimum as u64,
            );
        }
        return Ok(None);
    };
    let Some(hmb) = map(device_id, epoch, p) else {
        say(b"host memory buffer not offered: no page for its list", 0);
        return Ok(None);
    };
    let pages = hmb.pieces.len() as u32 * p.piece_pages;
    if hmb.pieces.is_empty() || pages < ask.minimum {
        say(b"host memory buffer not offered: pages mapped ", pages as u64);
        return Ok(None);
    }
    let dw = enable_dwords(pages, hmb.list.device_addr(), hmb.pieces.len() as u32);
    let what = "enable host memory buffer";
    let stride = info.doorbell_stride();
    match admin.set_features(regs, stride, FID_HOST_MEMORY_BUFFER, dw, what, ENABLE_TIMEOUT_MS) {
        Ok(()) => {
            say(b"host memory buffer given, pages ", pages as u64);
            Ok(Some(hmb))
        }
        // An error completion: the controller never took the memory.
        Err(e) if !ends_attempt(e) => {
            say(b"host memory buffer refused; served without, pages asked ", pages as u64);
            Ok(None)
        }
        Err(e) => {
            hmb.release(regs, info);
            Err(e)
        }
    }
}
