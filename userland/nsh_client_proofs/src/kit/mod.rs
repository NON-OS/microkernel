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

//! The test kit: deterministic randomness, the fleet's keys, a stand-in
//! attestation document and its checker, builders for the signed bodies,
//! a test server made from nsh_wire and nsh_crypto, and a record builder.

pub mod attest;
pub mod doc;
pub mod enrol;
pub mod fleet;
pub mod fuzz_oracle;
pub mod fuzz_proof;
pub mod fuzz_run;
pub mod grant;
pub mod keys;
pub mod keys_use;
pub mod pair;
pub mod pair_login;
pub mod proof;
pub mod record;
pub mod record_entry;
pub mod record_forge;
pub mod record_head;
pub mod record_sign;
pub mod rng;
pub mod server;
pub mod server_auth;
pub mod server_open;
pub mod sig;

pub const FLEET: [u8; 16] = [0xF1; 16];
pub const HOST: &[u8] = b"db-1.eu.example";
pub const AK: [u8; 64] = [0xAC; 64];
pub const PCRS: [[u8; 32]; 4] = [[0x00; 32], [0x01; 32], [0x02; 32], [0x07; 32]];
/// The client's clock in the good run; the enrolment runs out later.
pub const NOW: u64 = 5_000_000;
pub const NOT_AFTER: u64 = 9_000_000;

pub fn put16(out: &mut Vec<u8>, bytes: &[u8]) {
    out.extend_from_slice(&u16::try_from(bytes.len()).expect("bytes16").to_le_bytes());
    out.extend_from_slice(bytes);
}

pub fn put32(out: &mut Vec<u8>, bytes: &[u8]) {
    out.extend_from_slice(&u32::try_from(bytes.len()).expect("bytes32").to_le_bytes());
    out.extend_from_slice(bytes);
}

pub fn put8(out: &mut Vec<u8>, bytes: &[u8]) {
    out.push(u8::try_from(bytes.len()).expect("bytes8"));
    out.extend_from_slice(bytes);
}
