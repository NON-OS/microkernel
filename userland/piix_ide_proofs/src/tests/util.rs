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

use nonos_libc::{script, sent, Script};

use crate::discover::Found;
use crate::model::{install, Drive, Shared};
use crate::server::endpoint::REPLY_ENDPOINT;
use crate::server::{serve, TX_LEN};
use crate::setup::{run, Driver};
use crate::wire::HDR_LEN;

pub const DEVICE: u64 = 3;
pub const SENDER: u32 = 9;
pub const KERNEL: u32 = 0;

pub type Slots = [[Option<Drive>; 2]; 2];

/// QEMU's pc machine with `-hda` and a CD: a disk on the primary master,
/// a CD drive on the secondary master.
pub fn qemu(sectors: u64) -> Slots {
    [[Some(Drive::disk(sectors, true)), None], [Some(Drive::packet()), None]]
}

pub fn up_with(slots: Slots, s: Script, ctl_off: u16) -> (Result<Driver, &'static str>, Shared) {
    script(s);
    let p = install(slots, ctl_off);
    (run(Found { device_id: DEVICE, channels: [true, true] }), p)
}

pub fn up(slots: Slots) -> (Driver, Shared) {
    let (d, p) = up_with(slots, Script::default(), 0);
    (d.expect("bring-up"), p)
}

/// An NBW1 request: magic, version, op, flags, unit, id, payload length,
/// payload.
pub fn request(op: u16, unit: u16, id: u32, payload: &[u8]) -> Vec<u8> {
    let mut m = Vec::new();
    m.extend_from_slice(&0x4e42_5731u32.to_le_bytes());
    m.extend_from_slice(&1u16.to_le_bytes());
    m.extend_from_slice(&op.to_le_bytes());
    m.extend_from_slice(&0x0003u16.to_le_bytes());
    m.extend_from_slice(&unit.to_le_bytes());
    m.extend_from_slice(&id.to_le_bytes());
    m.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    m.extend_from_slice(payload);
    m
}

pub fn rw(lba: u64, count: u32, data: &[u8]) -> Vec<u8> {
    let mut b = lba.to_le_bytes().to_vec();
    b.extend_from_slice(&count.to_le_bytes());
    b.extend_from_slice(data);
    b
}

/// Serve `msg` from `sender` and return the one reply it got, which goes to
/// the driver's reply endpoint.
pub fn ask_from(d: &mut Driver, sender: u32, msg: &[u8]) -> Vec<u8> {
    let mut tx = vec![0u8; TX_LEN];
    serve(d, sender, msg, &mut tx);
    let mut r = sent();
    assert_eq!(r.len(), 1, "one reply per request");
    let (to, bytes) = r.remove(0);
    assert_eq!(to, REPLY_ENDPOINT, "the reply goes to endpoint.4294967355");
    let payload = u32::from_le_bytes(bytes[16..20].try_into().unwrap()) as usize;
    assert_eq!(bytes.len(), HDR_LEN + payload, "the header names what follows it");
    bytes
}

pub fn ask(d: &mut Driver, msg: &[u8]) -> Vec<u8> {
    ask_from(d, KERNEL, msg)
}

pub fn status(reply: &[u8]) -> i32 {
    i32::from_le_bytes(reply[HDR_LEN..HDR_LEN + 4].try_into().unwrap())
}

pub fn body(reply: &[u8]) -> &[u8] {
    &reply[HDR_LEN + 4..]
}

/// A sector whose bytes name it.
pub fn pattern(lba: u64) -> [u8; 512] {
    core::array::from_fn(|i| (lba as u8).wrapping_mul(31) ^ i as u8)
}
