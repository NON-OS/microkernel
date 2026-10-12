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

//! The server's discovery answers: as many entries as fit the MTU, all of
//! one length, or Attribute Not Found when nothing in range matches.

use super::server::{Peripheral, MTU};
use nonos_bt_wire::att::{error::*, opcode::*};

type Answer = Result<Vec<u8>, (u16, u8)>;

impl Peripheral {
    pub(super) fn find_by_type(&self, start: u16, end: u16, kind: &[u8], value: &[u8]) -> Answer {
        let attrs = &self.db.attrs;
        let last = attrs.last().map_or(0, |a| a.handle);
        let mut out = vec![FIND_BY_TYPE_VALUE_RSP];
        for (i, a) in attrs.iter().enumerate() {
            let in_range = self.repeat || (start..=end).contains(&a.handle);
            let hit = in_range && a.kind == kind && a.value == value;
            if !hit || out.len() + 4 > MTU {
                continue;
            }
            let group_end = attrs[i + 1..]
                .iter()
                .find(|n| n.kind == [0x00, 0x28])
                .map_or(last, |n| n.handle - 1);
            out.extend([a.handle.to_le_bytes(), group_end.to_le_bytes()].concat());
        }
        if out.len() == 1 {
            Err((start, ATTRIBUTE_NOT_FOUND))
        } else {
            Ok(out)
        }
    }

    pub(super) fn read_by_type(&self, start: u16, end: u16, kind: &[u8]) -> Answer {
        let mut each = 0;
        let mut out = vec![READ_BY_TYPE_RSP, 0];
        for a in
            self.db.attrs.iter().filter(|a| (start..=end).contains(&a.handle) && a.kind == kind)
        {
            let len = 2 + a.value.len();
            if (each != 0 && len != each) || out.len() + len > MTU {
                break;
            }
            each = len;
            out.extend([&a.handle.to_le_bytes()[..], &a.value].concat());
        }
        out[1] = each as u8;
        if each == 0 {
            Err((start, ATTRIBUTE_NOT_FOUND))
        } else {
            Ok(out)
        }
    }
}
