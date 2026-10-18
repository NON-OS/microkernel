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

//! load_store payloads: a head of caps and five lengths, and the blobs it names.

/// caps | five u32 lengths: the load_store head alone.
pub fn head(caps: u64, lens: [u32; 5]) -> Vec<u8> {
    let mut h = caps.to_le_bytes().to_vec();
    lens.iter().for_each(|l| h.extend_from_slice(&l.to_le_bytes()));
    h
}

/// The head followed by blob i as lens[i] bytes of 0xA0 + i.
pub fn store_payload(caps: u64, lens: [u32; 5]) -> Vec<u8> {
    let mut p = head(caps, lens);
    (0..5).for_each(|i| p.resize(p.len() + lens[i] as usize, 0xA0 + i as u8));
    p
}
