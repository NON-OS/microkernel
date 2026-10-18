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

//! Event packets the simulated controller sends, laid out by hand: code,
//! parameter length, parameters (Vol 4, Part E, 5.4.4).

/// Command Complete (Vol 4, Part E, 7.7.14): count, opcode, status, then
/// the return parameters.
pub fn complete(num: u8, op: u16, status: u8, ret: &[u8]) -> Vec<u8> {
    let [o0, o1] = op.to_le_bytes();
    let mut e = vec![0x0e, (4 + ret.len()) as u8, num, o0, o1, status];
    e.extend_from_slice(ret);
    e
}

/// Command Status (Vol 4, Part E, 7.7.15): status, count, opcode.
pub fn status(status: u8, num: u8, op: u16) -> Vec<u8> {
    let [o0, o1] = op.to_le_bytes();
    vec![0x0f, 4, status, num, o0, o1]
}

/// An LE Advertising Report (Vol 4, Part E, 7.7.65.2) with one report:
/// event type 0 (connectable undirected), a public address, `data`, and
/// RSSI -60.
pub fn adv_report(addr_last: u8, data: &[u8]) -> Vec<u8> {
    let mut sub = vec![0x02, 1, 0x00, 0x00, addr_last, 2, 3, 4, 5, 6];
    sub.push(data.len() as u8);
    sub.extend_from_slice(data);
    sub.push(-60i8 as u8);
    let mut e = vec![0x3e, sub.len() as u8];
    e.extend_from_slice(&sub);
    e
}

/// Number of Completed Packets (Vol 4, Part E, 7.7.19) for one handle.
pub fn completed(handle: u16, packets: u16) -> Vec<u8> {
    let [h0, h1] = handle.to_le_bytes();
    let [p0, p1] = packets.to_le_bytes();
    vec![0x13, 5, 1, h0, h1, p0, p1]
}
