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

//! HEST and its GHES structures, laid out as ACPI 6.5, 18.3.2 gives them.

pub fn header(sig: &[u8; 4], body: &[u8]) -> Vec<u8> {
    let mut t = sig.to_vec();
    t.extend_from_slice(&((36 + body.len()) as u32).to_le_bytes());
    t.resize(36, 0);
    [t, body.to_vec()].concat()
}

/// A GAS in system memory at `address`, 64 bits wide.
pub fn gas(address: u64) -> Vec<u8> {
    [vec![0, 64, 0, 4], address.to_le_bytes().to_vec()].concat()
}

/// GHES (or v2 with `ack`), notified by `notify`, polled every `poll`.
pub fn ghes(id: u16, status_at: u64, notify: u8, poll: u32, ack: Option<u64>) -> Vec<u8> {
    let kind: u16 = if ack.is_some() { 10 } else { 9 };
    let mut s = [kind.to_le_bytes(), id.to_le_bytes(), [0xFF, 0xFF]].concat();
    s.extend_from_slice(&[0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0]);
    s.extend(gas(status_at));
    let mut n = vec![notify, 28, 0, 0];
    n.extend_from_slice(&poll.to_le_bytes());
    n.resize(28, 0);
    s.extend(n);
    s.extend_from_slice(&0x1000u32.to_le_bytes());
    if let Some(a) = ack {
        s.extend(gas(a));
        s.extend_from_slice(&(!1u64).to_le_bytes());
        s.extend_from_slice(&1u64.to_le_bytes());
    }
    s
}
