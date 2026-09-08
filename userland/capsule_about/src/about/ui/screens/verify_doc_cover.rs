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

//! What the signature actually covers, as one line.

use super::append::put;

// "44 capsules over 143 bytes, 72 byte signature" as one cell. The three numbers
// are only meaningful next to each other, and each on its own row would read as a
// separate finding rather than as the shape of one document.
pub(super) fn counted(out: &mut [u8; 64], capsules: &[u8], signed: &[u8], sig: &[u8]) -> usize {
    let mut n = 0;
    n += put(&mut out[n..], capsules);
    n += put(&mut out[n..], b" capsules over ");
    n += put(&mut out[n..], signed);
    n += put(&mut out[n..], b" bytes, ");
    n += put(&mut out[n..], sig);
    n += put(&mut out[n..], b" byte signature");
    n
}
