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

//! Where the output stream goes: the engine it runs on, the outputs it
//! reaches, and whether the controller can address its memory.

use nonos_libc::DmaMapOut;

use crate::controller::codec::pincfg::OutKind;
use crate::controller::codec::plan::Plan;
use crate::controller::{layout, ControllerInfo, StreamDescriptor, STREAM_BIDI, STREAM_OUTPUT};
use crate::error::{HdaError, HdaResult};
use crate::protocol::{OUT_HEADPHONE, OUT_LINE, OUT_SPEAKER};

const FOUR_GIB: u64 = 1 << 32;

pub(super) fn outputs(plan: &Plan) -> u8 {
    let mut b = 0u8;
    if plan.has(OutKind::Speaker) {
        b |= OUT_SPEAKER;
    }
    if plan.has(OutKind::Headphone) {
        b |= OUT_HEADPHONE;
    }
    if plan.has(OutKind::LineOut) {
        b |= OUT_LINE;
    }
    b
}

/// A controller without GCAP.64OK drives only 32 address bits; a buffer
/// above 4 GiB would be fetched from its low half, someone else's memory.
pub(super) fn reachable(addr64: u8, maps: &[&DmaMapOut]) -> HdaResult<()> {
    if addr64 != 0 {
        return Ok(());
    }
    if maps.iter().any(|m| m.device_addr.saturating_add(m.length) > FOUR_GIB) {
        return Err(HdaError::DmaOutOfReach);
    }
    Ok(())
}

pub(super) fn output_descriptor(info: ControllerInfo) -> HdaResult<StreamDescriptor> {
    let (descs, n) = layout(info);
    let mut i = 0usize;
    while i < n {
        if descs[i].kind == STREAM_OUTPUT {
            return Ok(descs[i]);
        }
        i += 1;
    }
    // A controller with no output engine plays through a bidirectional one.
    descs[..n]
        .iter()
        .find(|d| d.kind == STREAM_BIDI)
        .copied()
        .ok_or(HdaError::UnsupportedController)
}
