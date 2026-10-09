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

//! The PE fields Authenticode reads, every offset checked against the file.

use super::error::PeError;
use super::field::{cert_table, u16_at, u32_at};

/// The PE format's own limit on the section count.
pub const MAX_SECTIONS: usize = 96;
const PE32: u16 = 0x10B;
const PE32_PLUS: u16 = 0x20B;
const SECURITY_DIR: u32 = 4;

/// Where Authenticode cuts the headers, and the section table.
pub(super) struct Layout {
    pub checksum: usize,
    /// The certificate table's data directory entry, if the image has one.
    pub cert_entry: Option<usize>,
    pub cert_size: usize,
    pub headers: usize,
    pub sections: usize,
    pub n_sections: usize,
}

pub(super) fn layout(f: &[u8]) -> Result<Layout, PeError> {
    if f.get(..2) != Some(b"MZ".as_slice()) {
        return Err(PeError::NotPe);
    }
    let pe = u32_at(f, 0x3C)? as usize;
    if f.get(pe..pe.checked_add(4).ok_or(PeError::Truncated)?) != Some(b"PE\0\0".as_slice()) {
        return Err(PeError::NotPe);
    }
    let n_sections = u16_at(f, pe + 6)? as usize;
    let opt_size = u16_at(f, pe + 20)? as usize;
    let opt = pe + 24;
    let (rva_at, dirs) = match u16_at(f, opt)? {
        PE32 => (92, 96),
        PE32_PLUS => (108, 112),
        _ => return Err(PeError::BadOptionalHeader),
    };
    if opt_size < dirs || n_sections > MAX_SECTIONS {
        return Err(if opt_size < dirs {
            PeError::BadOptionalHeader
        } else {
            PeError::TooManySections
        });
    }
    let n_rva = u32_at(f, opt + rva_at)?;
    let cert_entry = (n_rva > SECURITY_DIR).then_some(opt + dirs + 8 * SECURITY_DIR as usize);
    if cert_entry.is_some_and(|e| e + 8 > opt + opt_size) {
        return Err(PeError::BadOptionalHeader);
    }
    let cert_size = cert_table(f, cert_entry)?;
    let headers = u32_at(f, opt + 60)? as usize;
    let sections = opt + opt_size;
    let table_end = sections + 40 * n_sections;
    if headers > f.len() || table_end > f.len() || opt + 68 > headers {
        return Err(PeError::OutOfFile);
    }
    Ok(Layout { checksum: opt + 64, cert_entry, cert_size, headers, sections, n_sections })
}
