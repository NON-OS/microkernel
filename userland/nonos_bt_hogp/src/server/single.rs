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
//! The attribute database this host serves to its peripheral. GAP asks
//! The requests on one handle, or none: Exchange MTU (Vol 3, Part F,
//! 3.4.2.1), Read (3.4.4.3) and Write (3.4.5.1).

use super::db;
use super::serve::{Answer, ATT_MTU, BAD};
use nonos_bt_wire::att::{error::*, opcode::*};

/// Exchange MTU Response with this server's receive MTU (3.4.2.2).
pub fn exchange_mtu(p: &[u8]) -> Answer {
    if p.len() != 2 {
        return Err(BAD);
    }
    Ok([&[EXCHANGE_MTU_RSP][..], &(ATT_MTU as u16).to_le_bytes()].concat())
}

/// Read Response with as much of the value as fits (3.4.4.4).
pub fn read(p: &[u8]) -> Answer {
    let handle = u16::from_le_bytes(p.try_into().map_err(|_| BAD)?);
    let a = db::at(handle).ok_or((handle, INVALID_HANDLE))?;
    Ok([&[READ_RSP][..], &a.value[..a.value.len().min(ATT_MTU - 1)]].concat())
}

/// No attribute here can be written (3.4.5.1).
pub fn write(p: &[u8]) -> Answer {
    let handle = u16::from_le_bytes(p.get(..2).ok_or(BAD)?.try_into().map_err(|_| BAD)?);
    Err((handle, if db::at(handle).is_some() { WRITE_NOT_PERMITTED } else { INVALID_HANDLE }))
}
