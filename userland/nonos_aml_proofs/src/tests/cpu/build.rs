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

//! Encoders for the processor objects' packages.

use crate::enc::*;

/// ResourceTemplate { Register(space, width, offset, address, access) }.
pub fn reg(space: u8, width: u8, offset: u8, access: u8, address: u64) -> Vec<u8> {
    let mut t = vec![0x82, 0x0C, 0x00, space, width, offset, access];
    t.extend_from_slice(&address.to_le_bytes());
    t.extend_from_slice(&[0x79, 0x00]);
    buffer(t.len() as u64, &t)
}

pub fn ints(v: &[u64]) -> Vec<u8> {
    let e: Vec<Vec<u8>> = v.iter().map(|&x| int(x)).collect();
    package(e.len() as u8, &e)
}

pub fn cstate(register: Vec<u8>, kind: u64, latency: u64, power: u64) -> Vec<u8> {
    package(4, &[register, int(kind), int(latency), int(power)])
}

/// _CPC revision 3: highest 255, nominal 200, lowest nonlinear 100, lowest
/// 10, guaranteed the null register, desired a PCC register, zeros, then
/// reference performance 150, lowest 400 MHz and nominal 2000 MHz.
pub fn cpc() -> Vec<u8> {
    let mut e = vec![int(23), int(3), int(255), int(200), int(100), int(10)];
    e.push(reg(0, 0, 0, 0, 0));
    e.push(reg(0x0A, 32, 0, 0, 0x14));
    e.extend((8..20).map(|_| int(0)));
    e.extend([int(150), int(400), int(2000)]);
    package(e.len() as u8, &e)
}
