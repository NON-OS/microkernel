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

//! A request as the host sends it. The I2C address phase carries the
//! display's address; the bytes after it are the host's source address, a
//! length byte (0x80 and the count), the message and a checksum: the XOR
//! of every byte, the display's write address (0x6E) included.

/// The 7-bit I2C address DDC/CI is spoken on (0x6E written, 0x6F read).
pub const DDC_CI_ADDR: u32 = 0x37;
pub const DISPLAY: u8 = (DDC_CI_ADDR as u8) << 1;
/// The host's source address.
pub const HOST: u8 = 0x51;
/// A reply's checksum starts from this, standing for the host.
pub const REPLY_SEED: u8 = 0x50;

pub const GET_VCP: u8 = 0x01;
pub const GET_VCP_REPLY: u8 = 0x02;
pub const SET_VCP: u8 = 0x03;

pub fn xor(seed: u8, bytes: &[u8]) -> u8 {
    bytes.iter().fold(seed, |a, b| a ^ b)
}

/// Get VCP Feature for `code`: what is sent after the address.
pub fn get_request(code: u8) -> [u8; 5] {
    let mut r = [HOST, 0x82, GET_VCP, code, 0];
    r[4] = xor(DISPLAY, &r[..4]);
    r
}

/// Set VCP Feature: `code` to `value`, high byte first.
pub fn set_request(code: u8, value: u16) -> [u8; 7] {
    let [hi, lo] = value.to_be_bytes();
    let mut r = [HOST, 0x84, SET_VCP, code, hi, lo, 0];
    r[6] = xor(DISPLAY, &r[..6]);
    r
}
