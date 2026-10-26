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
//! The header check the kernel makes before it walks DMAR or IVRS (their
//! parse_* entries, through the SDT's validate_checksum), for the sweeps
//! that feed those walks.

/// What the kernel's checksum check admits (dmar.rs, ivrs.rs): a declared length from
/// the header to the bytes handed over, summing to zero over that length.
pub fn admitted(t: &[u8]) -> Option<&[u8]> {
    let n = u32::from_le_bytes(t.get(4..8)?.try_into().ok()?) as usize;
    let t = t.get(..n).filter(|_| n >= 36)?;
    (t.iter().fold(0u8, |s, b| s.wrapping_add(*b)) == 0).then_some(t)
}
