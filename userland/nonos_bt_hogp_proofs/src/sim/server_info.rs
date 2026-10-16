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

use super::server::Peripheral;
use nonos_bt_wire::att::{error::*, opcode::*};

impl Peripheral {
    pub(super) fn find_information(&self, start: u16, end: u16) -> Result<Vec<u8>, (u16, u8)> {
        let mut format = 0;
        let mut out = vec![FIND_INFORMATION_RSP, 0];
        for a in
            self.db.attrs.iter().filter(|a| a.handle >= start && (self.wide || a.handle <= end))
        {
            let f = if a.kind.len() == 2 { 1 } else { 2 };
            if (format != 0 && f != format) || out.len() + 2 + a.kind.len() > self.mtu {
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

impl Peripheral {
    /// Read (Vol 3, Part F, 3.4.4.3): as much of the value as fits. Read
    /// Blob (3.4.4.5): as much as fits from an offset, past the end being
    /// Invalid Offset, and at the end `blob_end` if set.
    pub(super) fn read(&self, handle: u16, offset: Option<u16>) -> Result<Vec<u8>, (u16, u8)> {
        let a = self.db.attrs.iter().find(|a| a.handle == handle);
        let a = a.ok_or((handle, INVALID_HANDLE))?;
        let (op, at) = offset.map_or((READ_RSP, 0), |o| (READ_BLOB_RSP, usize::from(o)));
        let end = self.blob_end.filter(|_| offset.is_some() && at == a.value.len());
        if let Some(code) = end {
            return Err((handle, code));
        }
        let rest = a.value.get(at..).ok_or((handle, INVALID_OFFSET))?;
        Ok([&[op][..], &rest[..rest.len().min(self.mtu - 1)]].concat())
    }
}
