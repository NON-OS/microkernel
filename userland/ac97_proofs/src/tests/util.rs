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

//! A controller and a driver up on it, and requests built by hand.

use nonos_libc::replies;

use crate::discover::Found;
use crate::model::{install, Options, Shared};
use crate::protocol::HDR_LEN;
use crate::server::{serve, Stream};
use crate::setup::{run, Driver};

pub const DEVICE: u64 = 3;
pub const SENDER: u32 = 9;

pub fn up_with(opt: Options) -> (Result<Driver, &'static str>, Shared) {
    let ich = install(opt);
    (run(Found { device_id: DEVICE }), ich)
}

pub fn up() -> (Driver, Shared) {
    let (d, ich) = up_with(Options::default());
    (d.expect("bring-up"), ich)
}

/// An NHDA request: magic, version, op, flags, id, payload length, payload.
pub fn request(op: u16, id: u32, payload: &[u8]) -> Vec<u8> {
    let mut m = Vec::new();
    m.extend_from_slice(&0x4e48_4441u32.to_le_bytes());
    m.extend_from_slice(&1u16.to_le_bytes());
    m.extend_from_slice(&op.to_le_bytes());
    m.extend_from_slice(&0x0005u16.to_le_bytes());
    m.extend_from_slice(&0u16.to_le_bytes());
    m.extend_from_slice(&id.to_le_bytes());
    m.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    m.extend_from_slice(payload);
    m
}

/// Serve `msg` and return the one reply it got.
pub fn ask(d: &mut Driver, st: &mut Stream, msg: &[u8]) -> Vec<u8> {
    let mut tx = [0u8; 64];
    serve(d, st, SENDER, msg, &mut tx);
    let mut r = replies();
    assert_eq!(r.len(), 1, "one reply per request");
    let (to, bytes) = r.remove(0);
    assert_eq!(to, SENDER, "the reply goes to the caller");
    bytes
}

pub fn status(reply: &[u8]) -> i32 {
    i32::from_le_bytes(reply[HDR_LEN..HDR_LEN + 4].try_into().unwrap())
}
