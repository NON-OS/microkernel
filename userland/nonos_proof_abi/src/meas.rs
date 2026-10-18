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

//! M, the chain of measurements a supervisor had the kernel verify.

use alloc::vec::Vec;

pub const MEAS_DOMAIN: &[u8] = b"NONOS-FAMILY-MEAS-v1";

/// One link: M = BLAKE3(domain | M | measurement).
pub fn meas_step(m: &[u8; 32], measurement: &[u8; 32]) -> [u8; 32] {
    let mut b = Vec::with_capacity(MEAS_DOMAIN.len() + 64);
    b.extend_from_slice(MEAS_DOMAIN);
    b.extend_from_slice(m);
    b.extend_from_slice(measurement);
    *blake3::hash(&b).as_bytes()
}

/// The whole chain from M0 = zero, in the order the list keeps.
pub fn meas_chain(list: &[[u8; 32]]) -> [u8; 32] {
    list.iter().fold([0u8; 32], |m, x| meas_step(&m, x))
}
