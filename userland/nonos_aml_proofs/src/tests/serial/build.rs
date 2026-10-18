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

//! Serial Bus and GPIO Connection descriptors as 6.4.3.8 lays them out.

use crate::enc::*;

fn large(tag: u8, body: Vec<u8>) -> Vec<u8> {
    let len = body.len() as u16;
    [vec![tag, len as u8, (len >> 8) as u8], body].concat()
}

/// A serial bus descriptor of `kind` with type flags and type data.
pub fn serial(kind: u8, tflags: u16, data: &[u8], source: &str) -> Vec<u8> {
    let tlen = data.len() as u16;
    let mut b = vec![2, 0, kind, 0x02, tflags as u8, (tflags >> 8) as u8, 1];
    b.extend_from_slice(&tlen.to_le_bytes());
    b.extend_from_slice(data);
    b.extend_from_slice(source.as_bytes());
    b.push(0);
    large(0x8E, b)
}

pub fn i2c(address: u16, speed: u32, ten_bit: bool, source: &str) -> Vec<u8> {
    let data = [speed.to_le_bytes().as_slice(), &address.to_le_bytes()].concat();
    serial(1, ten_bit as u16, &data, source)
}

/// GpioInt (`interrupt`) or GpioIo, with its flags word, pins and source.
pub fn gpio(interrupt: bool, flags: u16, pins: &[u16], source: &str) -> Vec<u8> {
    let pin_at = 23u16;
    let name_at = pin_at + 2 * pins.len() as u16;
    let vendor_at = name_at + source.len() as u16 + 1;
    let mut w = vec![1, !interrupt as u8, 0, 0, flags as u8, (flags >> 8) as u8, 1, 0, 0, 10, 0];
    for v in [pin_at.to_le_bytes(), [0, name_at as u8], [(name_at >> 8) as u8, vendor_at as u8]] {
        w.extend_from_slice(&v);
    }
    w.extend_from_slice(&[(vendor_at >> 8) as u8, 0, 0]);
    pins.iter().for_each(|p| w.extend_from_slice(&p.to_le_bytes()));
    w.extend_from_slice(source.as_bytes());
    w.push(0);
    large(0x8C, w)
}

pub fn template(parts: &[Vec<u8>]) -> Vec<u8> {
    let t = [parts.concat(), vec![0x79, 0]].concat();
    buffer(t.len() as u64, &t)
}
