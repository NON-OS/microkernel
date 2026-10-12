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

//! The ASL the idioms need beyond the shared encoders (ACPI 6.5, 20.2).

use crate::enc::*;

pub fn mutex(n: &str, sync: u8) -> Vec<u8> {
    cat(&[&op(0x5B01), &name(n), &[sync]])
}

pub fn acquire(n: &str, timeout: u16) -> Vec<u8> {
    e(0x5B23, &[&name(n), &timeout.to_le_bytes()])
}

pub fn release(n: &str) -> Vec<u8> {
    e(0x5B27, &[&name(n)])
}

pub fn while_(pred: &[u8], body: &[u8]) -> Vec<u8> {
    pkg(0xA2, &cat(&[pred, body]))
}

/// A binary operator with no target (`Zero`, the NullName).
pub fn bin(code: u16, a: &[u8], b: &[u8]) -> Vec<u8> {
    e(code, &[a, b, &[0x00]])
}

pub fn increment(target: &[u8]) -> Vec<u8> {
    e(0x75, &[target])
}

pub fn notify(obj: &str, value: u64) -> Vec<u8> {
    e(0x86, &[&name(obj), &int(value)])
}

/// A call: the method's name, then its arguments.
pub fn invoke(path: &str, args: &[Vec<u8>]) -> Vec<u8> {
    cat(&[&name(path), &args.concat()])
}

pub fn sleep(ms: u64) -> Vec<u8> {
    e(0x5B22, &[&int(ms)])
}

pub const LAND: u16 = 0x90;
pub const LEQUAL: u16 = 0x93;
pub const LLESS: u16 = 0x95;
pub const AND: u16 = 0x7B;
pub const ADD: u16 = 0x72;
