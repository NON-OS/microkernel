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

//! An object's path: `/u/<low byte as 2 hex>/<number as 16 hex>`. The low
//! byte spreads objects over 256 directories, so no directory's chain of
//! entry blocks grows long enough to slow every lookup in it.

/// The most one object holds: one chunk of the broker's files.
pub const OBJECT_MAX: usize = 1 << 20;

pub(super) const ROOT: &[u8] = b"/u";

/// `/u/xx`, the object's directory.
pub(super) fn dir_of(id: u64) -> [u8; 5] {
    let mut out = *b"/u/00";
    hex(&[id as u8], &mut out[3..5]);
    out
}

/// `/u/xx/<16 hex>`.
pub(super) fn path_of(id: u64) -> [u8; 22] {
    let mut out = [0u8; 22];
    out[..5].copy_from_slice(&dir_of(id));
    out[5] = b'/';
    hex(&id.to_be_bytes(), &mut out[6..]);
    out
}

fn hex(bytes: &[u8], out: &mut [u8]) {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    for (i, b) in bytes.iter().enumerate() {
        out[2 * i] = DIGITS[(b >> 4) as usize];
        out[2 * i + 1] = DIGITS[(b & 15) as usize];
    }
}
