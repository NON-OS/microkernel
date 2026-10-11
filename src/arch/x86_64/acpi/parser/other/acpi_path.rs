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

//! ACPI namespace paths as firmware writes them in different tables: the
//! same object may be spelled with or without its padding. Pure.

/// Two namespace paths name the same object: the root prefix is dropped and
/// each segment is compared padded to four characters with `_`, since
/// firmware writes `\_SB.PCI0.SDHA` and `\_SB_.PCI0.SDHA` for one device.
pub fn same_path(a: &[u8], b: &[u8]) -> bool {
    let mut x = strip(a).split(|c| *c == b'.');
    let mut y = strip(b).split(|c| *c == b'.');
    loop {
        match (x.next(), y.next()) {
            (None, None) => return true,
            (Some(s), Some(t)) if padded(s).is_some() && padded(s) == padded(t) => {}
            _ => return false,
        }
    }
}

fn strip(path: &[u8]) -> &[u8] {
    path.strip_prefix(b"\\").unwrap_or(path)
}

fn padded(segment: &[u8]) -> Option<[u8; 4]> {
    if segment.is_empty() || segment.len() > 4 {
        return None;
    }
    let mut out = [b'_'; 4];
    out[..segment.len()].copy_from_slice(segment);
    Some(out)
}
