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

//! SET_VCP: one MCCS control written on one panel, passed to the driver,
//! which holds the list of controls a panel may have written.

use super::read::words;
use crate::protocol::SET_VCP_REQ_LEN;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetVcp {
    pub panel: u32,
    pub code: u8,
    pub value: u16,
}

/// The panel, a code that fits a byte and a value that fits 16 bits.
pub fn set_vcp(body: &[u8]) -> Option<SetVcp> {
    let [panel, code, value] = words::<3, SET_VCP_REQ_LEN>(body)?;
    let (code, value) = (u8::try_from(code).ok()?, u16::try_from(value).ok()?);
    Some(SetVcp { panel, code, value })
}
