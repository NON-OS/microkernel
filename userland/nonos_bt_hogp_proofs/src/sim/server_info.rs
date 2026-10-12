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

//! Find Information (Vol 3, Part F, 3.4.3.1): handles and types, all of
//! one format, as many as fit.

use super::server::{Peripheral, MTU};
use nonos_bt_wire::att::{error::*, opcode::*};

impl Peripheral {
    pub(super) fn find_information(&self, start: u16, end: u16) -> Result<Vec<u8>, (u16, u8)> {
        let mut format = 0;
        let mut out = vec![FIND_INFORMATION_RSP, 0];
        for a in
            self.db.attrs.iter().filter(|a| a.handle >= start && (self.wide || a.handle <= end))
        {
            let f = if a.kind.len() == 2 { 1 } else { 2 };
            if (format != 0 && f != format) || out.len() + 2 + a.kind.len() > MTU {
                break;
            }
            format = f;
            out.extend([&a.handle.to_le_bytes()[..], &a.kind].concat());
        }
        out[1] = format;
        if format == 0 {
            Err((start, ATTRIBUTE_NOT_FOUND))
        } else {
            Ok(out)
        }
    }
}
