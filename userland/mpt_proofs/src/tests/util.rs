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

//! An IOC, the driver up on it, and requests built by hand.

use nonos_libc::{mk_dma_map, sent, DmaMapOut, MK_DMA_MAP_COHERENT, MK_DMA_MAP_DMA32};
use nonos_scsi::Hba;

use crate::constants::layout::GRANT_BYTES;
use crate::hba::MptHba;
use crate::ioc;
use crate::model::sim::Unit;
use crate::model::{ioc as model, Mpt};
use crate::units::Units;
use crate::wire::HDR_LEN;

pub const REPLY: u64 = 4_294_967_359;

pub type Up = Units<MptHba<Mpt>>;

/// The grant as the shim's broker places it: bus 0x100000, host memory
/// behind it.
pub fn grant() -> DmaMapOut {
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    assert_eq!(mk_dma_map(3, 7, GRANT_BYTES, MK_DMA_MAP_DMA32 | MK_DMA_MAP_COHERENT, &mut out), 0);
    out
}

pub fn up_on(m: &Mpt) -> Up {
    let g = grant();
    let f = ioc::start(m, g.user_va, g.device_addr).expect("IOC up");
    let mut u = Units::new(
        MptHba::new(m.clone(), g.user_va, g.device_addr, f.max_devices),
        REPLY,
        b"driver.mpt0",
    );
    u.scan(f.max_devices as u16 - 1, 7);
    u
}

pub fn up(units: Vec<((u16, u16), Unit)>) -> (Up, Mpt) {
    let m = model(units);
    (up_on(&m), m)
}

pub fn request(op: u16, unit: u16, payload: &[u8]) -> Vec<u8> {
    let mut m = Vec::new();
    m.extend_from_slice(&0x4e42_5731u32.to_le_bytes());
    m.extend_from_slice(&1u16.to_le_bytes());
    m.extend_from_slice(&op.to_le_bytes());
    m.extend_from_slice(&0u16.to_le_bytes());
    m.extend_from_slice(&unit.to_le_bytes());
    m.extend_from_slice(&9u32.to_le_bytes());
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

pub fn ask(u: &mut Up, msg: &[u8]) -> Vec<u8> {
    let mut tx = vec![0u8; crate::server::tx_len(u.hba.buffer().len())];
    crate::server::serve(u, 0, msg, &mut tx);
    let mut r = sent();
    assert_eq!(r.len(), 1, "one reply per request");
    let (to, bytes) = r.remove(0);
    assert_eq!(to, REPLY);
    bytes
}

pub fn status(reply: &[u8]) -> i32 {
    i32::from_le_bytes(reply[HDR_LEN..HDR_LEN + 4].try_into().unwrap())
}

pub fn body(reply: &[u8]) -> &[u8] {
    &reply[HDR_LEN + 4..]
}
