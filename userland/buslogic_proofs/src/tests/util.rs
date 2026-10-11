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

//! An adapter, the driver up on it, and requests built by hand.

use nonos_libc::{script, sent, Script};
use nonos_scsi::Hba;

use crate::discover::Found;
use crate::model::sim::Unit;
use crate::model::{install, Shared};
use crate::setup::driver::Driver;
use crate::setup::run;
use crate::wire::HDR_LEN;
use crate::REPLY_ENDPOINT;

pub const DEVICE: u64 = 3;

pub fn up_with(
    units: Vec<((u16, u16), Unit)>,
    s: Script,
    tweak: impl FnOnce(&Shared),
) -> (Result<Driver, &'static str>, Shared) {
    script(s);
    let m = install(units);
    tweak(&m);
    (run(Found { device_id: DEVICE }), m)
}

pub fn up(units: Vec<((u16, u16), Unit)>) -> (Driver, Shared) {
    let (d, m) = up_with(units, Script::default(), |_| {});
    (d.expect("bring-up"), m)
}

pub fn request(op: u16, unit: u16, payload: &[u8]) -> Vec<u8> {
    let mut m = Vec::new();
    m.extend_from_slice(&0x4e42_5731u32.to_le_bytes());
    m.extend_from_slice(&1u16.to_le_bytes());
    m.extend_from_slice(&op.to_le_bytes());
    m.extend_from_slice(&0u16.to_le_bytes());
    m.extend_from_slice(&unit.to_le_bytes());
    m.extend_from_slice(&5u32.to_le_bytes());
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

pub fn ask(d: &mut Driver, msg: &[u8]) -> Vec<u8> {
    let mut tx = vec![0u8; crate::server::tx_len(d.units.hba.buffer().len())];
    crate::server::serve(&mut d.units, 0, msg, &mut tx);
    let mut r = sent();
    assert_eq!(r.len(), 1, "one reply per request");
    let (to, bytes) = r.remove(0);
    assert_eq!(to, REPLY_ENDPOINT);
    bytes
}

pub fn status(reply: &[u8]) -> i32 {
    i32::from_le_bytes(reply[HDR_LEN..HDR_LEN + 4].try_into().unwrap())
}

pub fn body(reply: &[u8]) -> &[u8] {
    &reply[HDR_LEN + 4..]
}
