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
use crate::error::{XhciError, XhciResult};
use nonos_libc::{mk_device_release, mk_mmio_map, MmioMapOut};
const BAR_INDEX: u32 = 0;
/// How much of BAR0 is mapped. It must reach the doorbell array and the
/// extended capabilities, which carry the legacy handoff and the port
/// protocols. Intel PCH controllers put DBOFF at 0x3000, so the 0x3000-byte
/// window this replaced failed `require_window` on every one of them.
/// HCCPARAMS1.xECP can point up to 256 KiB in, Intel puts the list near
/// 0x8000, and BAR0 is 64 KiB on every Intel controller; the cap only keeps
/// a large BAR from costing address space for registers nobody reads.
const REGISTER_WINDOW_LEN: u64 = 512 * 1024;
const PAGE_MASK: u64 = 0xFFF;
pub fn mmio_map(device_id: u64, claim_epoch: u64, bar0_size: u64) -> XhciResult<MmioMapOut> {
    let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    let length = bar0_size.min(REGISTER_WINDOW_LEN) & !PAGE_MASK;
    let r = mk_mmio_map(device_id, claim_epoch, BAR_INDEX, 0, 0, length, &mut out);
    if r < 0 {
        /* Nothing is mapped yet, so the claim is all there is to give back; the
         * caller returns at once and would otherwise keep the controller. */
        let _ = mk_device_release(device_id);
        return Err(XhciError::BrokerCallFailed(r));
    }
    Ok(out)
}
