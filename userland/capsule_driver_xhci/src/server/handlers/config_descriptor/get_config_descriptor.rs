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
use crate::controller::{issue_control_transfer, ControlRequest};
use crate::dma::DmaRegion;
use crate::error::{XhciError, XhciResult};
use crate::server::context::Context;

const GET_DESCRIPTOR: u8 = 6;
const CONFIGURATION: u16 = 2;

/// GET_DESCRIPTOR(Configuration `index`) for `len` bytes into `out`; the
/// bytes the device sent. Any index goes to the device, which answers one
/// it does not have with a STALL.
pub(super) fn get_config_descriptor(
    ctx: &mut Context,
    slot: u8,
    index: u8,
    out: &DmaRegion,
    len: u16,
) -> XhciResult<usize> {
    let d = &mut ctx.driver;
    let res =
        d.slots.resources_mut(slot, d.layout.max_slots).ok_or(XhciError::ControllerUnsupported)?;
    let request = ControlRequest {
        bm_request_type: 0x80,
        b_request: GET_DESCRIPTOR,
        w_value: CONFIGURATION << 8 | index as u16,
        w_index: 0,
        data_phys: out.phys(),
        data_len: len,
    };
    let (db, intr) = (d.layout.doorbell_base, d.layout.primary_intr_base);
    issue_control_transfer(db, intr, &mut d.event_ring, &mut res.ep0, slot, request)
        .map(usize::from)
}
