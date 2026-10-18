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

//! The events the air's controller sends that the HCI proofs' builders do
//! not cover with these addresses, laid out by hand (Vol 4, Part E, 5.4.4).

use super::state::{Air, HANDLE};

/// An LE Advertising Report (7.7.65.2) with one report from `from`, a
/// random address, of event type `kind`, with `data`; RSSI -50.
pub fn advertised(from: [u8; 6], kind: u8, data: &[u8]) -> Vec<u8> {
    let mut sub = vec![0x02, 1, kind, 0x01];
    sub.extend_from_slice(&from);
    sub.push(data.len() as u8);
    sub.extend_from_slice(data);
    sub.push(-50i8 as u8);
    let mut e = vec![0x3e, sub.len() as u8];
    e.extend_from_slice(&sub);
    e
}

/// LE Connection Complete (7.7.65.1) with `from`, as central, interval
/// 0x0020, latency 0, timeout 0x0190.
pub fn connected(from: [u8; 6]) -> Vec<u8> {
    let [h0, h1] = HANDLE.to_le_bytes();
    let mut e = vec![0x3e, 19, 0x01, 0x00, h0, h1, 0x00, 0x01];
    e.extend_from_slice(&from);
    e.extend_from_slice(&[0x20, 0x00, 0x00, 0x00, 0x90, 0x01, 0x00]);
    e
}

impl Air {
    /// An event as the device sends it: cut into 16-octet transfers.
    pub(super) fn event(&mut self, e: &[u8]) {
        self.interrupt.extend(e.chunks(16).map(<[u8]>::to_vec));
    }
}
