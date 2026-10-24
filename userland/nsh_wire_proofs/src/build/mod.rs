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

//! Builders for valid bodies. Each holds every field as plain data with a
//! valid default; `bytes()` lays them out as the specification says, so a
//! test changes one field and parses the result.

pub mod enrol;
pub mod grant;
mod grant_bytes;
pub mod proof;
pub mod record;
mod record_bytes;

pub const ED_PK: usize = 32;
pub const ED_SIG: usize = 64;
pub const ML_PK: usize = 1952;
pub const ML_SIG: usize = 3309;

pub fn put_u16(out: &mut Vec<u8>, v: u16) {
    out.extend_from_slice(&v.to_le_bytes());
}

pub fn put_u32(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_le_bytes());
}

pub fn put_u64(out: &mut Vec<u8>, v: u64) {
    out.extend_from_slice(&v.to_le_bytes());
}

pub fn put8(out: &mut Vec<u8>, bytes: &[u8]) {
    out.push(u8::try_from(bytes.len()).expect("bytes8 field over 255"));
    out.extend_from_slice(bytes);
}

pub fn put16(out: &mut Vec<u8>, bytes: &[u8]) {
    put_u16(out, u16::try_from(bytes.len()).expect("bytes16 field over 65535"));
    out.extend_from_slice(bytes);
}

pub fn put32(out: &mut Vec<u8>, bytes: &[u8]) {
    put_u32(out, u32::try_from(bytes.len()).expect("bytes32 field over u32"));
    out.extend_from_slice(bytes);
}

/// A hybrid signature whose bytes are all `fill`; the parsers place
/// signatures and never judge them, so any bytes do.
pub fn sig(out: &mut Vec<u8>, fill: u8) {
    out.extend(core::iter::repeat_n(fill, ED_SIG + ML_SIG));
}

pub fn public(out: &mut Vec<u8>, fill: u8) {
    out.extend(core::iter::repeat_n(fill, ED_PK + ML_PK));
}

/// A capability-name list as `CapNames` lays it out.
pub fn caps(out: &mut Vec<u8>, names: &[&[u8]]) {
    out.push(u8::try_from(names.len()).expect("too many names"));
    names.iter().for_each(|n| put8(out, n));
}
