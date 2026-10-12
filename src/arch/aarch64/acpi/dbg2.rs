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

//! DBG2, the debug port table: the ports firmware sets aside for a
//! debugger or a console, on Arm and on PC servers alike (a BMC's
//! serial-over-LAN UART is one). After the header: the offset u32 and
//! count u32 of the device structures (`dbg2_device`). Recalled from
//! Microsoft's specification, unverified. The walk stops at the first
//! structure that does not fit, and keeps at most `MAX_DEVICES`.

use super::dbg2_device::device;
pub use super::dbg2_device::Dbg2Device;
use super::le::u32_at;
use super::sdt::{table, TableError};
use alloc::vec::Vec;

pub const MAX_DEVICES: usize = 16;
pub const PORT_SERIAL: u16 = 0x8000;

pub fn dbg2(t: &[u8]) -> Result<Vec<Dbg2Device>, TableError> {
    let t = table(t, b"DBG2")?;
    let (at, count) = (u32_at(t, 36), u32_at(t, 40));
    let (Some(mut at), Some(count)) = (at.map(|a| a as usize), count) else {
        return Err(TableError::Short);
    };
    let mut out = Vec::new();
    for _ in 0..(count as usize).min(MAX_DEVICES) {
        let (d, len) = device(t, at)?;
        out.push(d);
        at += len;
    }
    Ok(out)
}
