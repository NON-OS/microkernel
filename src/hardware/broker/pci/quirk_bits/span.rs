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

//! How far a standard capability reaches, where its kind fixes it (PCI
//! Local Bus 3.0 section 6.8 for MSI, PCI PM 1.2 for Power Management, the
//! PCI Express Base for its own; lengths as recalled, unverified).

/// The length of the capabilities whose size is fixed by their kind; any
/// other reaches to the next entry, or to the end of the header space.
pub(super) fn span(read: &dyn Fn(u16) -> u8, at: u16) -> Option<u16> {
    let control = read(at + 2) as u16 | (read(at + 3) as u16) << 8;
    match read(at) {
        0x01 => Some(8),
        // MSI: 64-bit address (bit 7) and per-vector masking (bit 8).
        0x05 => Some(match (control & 0x80 != 0, control & 0x100 != 0) {
            (false, false) => 10,
            (true, false) => 14,
            (false, true) => 20,
            (true, true) => 24,
        }),
        0x09 => Some((control & 0xFF).max(3)),
        0x0A => Some(4),
        0x10 => Some(0x3C),
        0x11 => Some(12),
        _ => None,
    }
}
