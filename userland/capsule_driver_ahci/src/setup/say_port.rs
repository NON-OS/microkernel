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

//! The serial line each port tried leaves.

use crate::constants::regs::{
    PORT_BASE, PORT_CMD, PORT_SERR, PORT_SIG, PORT_SSTS, PORT_STRIDE, PORT_TFD,
};
use crate::engine::Port;
use crate::error::{reason, AhciError, AhciResult};
use crate::log::Line;
use crate::regs::Regs;

/// One serial line per port tried: what came of it, and the registers a
/// failure is read from.
pub(super) fn say_port(regs: Regs, index: u8, up: &AhciResult<Port>) {
    let base = PORT_BASE + u32::from(index) * PORT_STRIDE;
    let r = |off: u32| unsafe { regs.r32(base + off) };
    let mut line = Line::new();
    line.text(b"port ").num(u64::from(index)).text(b": ");
    match up {
        Ok(port) => line
            .text(b"disk, ")
            .num(port.capacity_sectors)
            .text(b" sectors, ")
            .text(port.names.model()),
        Err(AhciError::NoDisk) => line.text(b"no disk"),
        Err(e) => line.text(reason(*e).as_bytes()),
    };
    line.text(b" SSTS ")
        .hex(r(PORT_SSTS))
        .text(b" TFD ")
        .hex(r(PORT_TFD))
        .text(b" SERR ")
        .hex(r(PORT_SERR))
        .text(b" SIG ")
        .hex(r(PORT_SIG))
        .text(b" CMD ")
        .hex(r(PORT_CMD))
        .send();
}
