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

//! Builders for valid bodies, and the setting every test starts from: one
//! fleet, one clock reading, and the keys in `keys`. Each builder holds its
//! fields as plain data with a valid default; `bytes()` lays them out and
//! signs them, so a test changes one field and checks the result.

mod approval;
mod escalation;
pub mod grant;
mod grant_bytes;
pub mod keys;
pub mod revoke;
mod role;
pub mod run;
mod run_esc;
mod spoil;
mod stream;
pub mod table;
mod table_bytes;
pub mod user_auth;

pub use spoil::Spoil;

pub const FLEET: [u8; 16] = [0xF1; 16];
pub const HOUR: u64 = 3_600_000;
/// The host clock in every test, in milliseconds.
pub const NOW: u64 = 1_800_000_000_000;
/// The transcript digest the user signs over.
pub const TH: [u8; 64] = [0x7A; 64];
pub const BASE_ID: [u8; 16] = [0x11; 16];
pub const ESC_ID: [u8; 16] = [0x22; 16];

pub fn put_u16(out: &mut Vec<u8>, v: u16) {
    out.extend_from_slice(&v.to_le_bytes());
}

pub fn put_u64(out: &mut Vec<u8>, v: u64) {
    out.extend_from_slice(&v.to_le_bytes());
}

pub fn put8(out: &mut Vec<u8>, bytes: &[u8]) {
    out.push(u8::try_from(bytes.len()).expect("bytes8 field over 255"));
    out.extend_from_slice(bytes);
}

/// A capability-name list in ascending order, as `CapNames` lays it out.
pub fn caps(out: &mut Vec<u8>, names: &[&str]) {
    let mut sorted = names.to_vec();
    sorted.sort_unstable();
    out.push(u8::try_from(sorted.len()).expect("too many names"));
    sorted.iter().for_each(|n| put8(out, n.as_bytes()));
}
