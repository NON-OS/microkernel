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

//! Whether bytes read are whole TLS records end to end: what tells an answer
//! cut partway through a record from one whose records fail to open.

/// True when `bytes` is a run of whole records, each a five-byte header and
/// the length it names, with nothing left over.
pub fn records_whole(bytes: &[u8]) -> bool {
    let mut pos = 0usize;
    while pos < bytes.len() {
        let Some(header) = bytes.get(pos..pos + 5) else { return false };
        let len = u16::from_be_bytes([header[3], header[4]]) as usize;
        pos += 5 + len;
    }
    pos == bytes.len()
}
