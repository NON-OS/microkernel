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

//! A host, the driver's channel opened on it and its protocol started, the
//! way the capsule's setup does it.

use nonos_vmbus::Pages;

use crate::constants::channel::{gpadl_id, MAX_LUN, MAX_TARGET, OPEN_USER};
use crate::constants::layout::{DATA_OFF, GRANT_BYTES, PAGE, RINGS, RING_BYTES};
use crate::hba::VmHba;
use crate::model::sim::Unit;
use crate::model::{Grant, Seen, Server, StorHost, BASE_PFN};
use crate::start;
use crate::units::Units;

pub const RELID: u32 = 14;
pub const REPLY: u64 = 4_294_967_394;

pub type StorHba = VmHba<StorHost, Pages>;
pub type Up = Units<StorHba>;

/// The channel open on a fresh grant, the protocol not yet started.
pub fn opened(server: Server) -> StorHba {
    let grant = Grant::new(GRANT_BYTES as usize);
    let mut host = StorHost::new(grant, server);
    let mut rings = grant.pages(RING_BYTES);
    let chan = nonos_vmbus::open(&mut host, &mut rings, RELID, gpadl_id(RELID), BASE_PFN, RINGS, &OPEN_USER)
        .expect("the channel opens");
    // SAFETY: the grant's data buffer, leaked for the thread.
    let data = unsafe { grant.va.add(DATA_OFF) };
    // SAFETY: as above; the model writes it only inside a request.
    unsafe { VmHba::new(host, rings, chan, data, BASE_PFN + (DATA_OFF / PAGE) as u64) }
}

pub fn started(server: Server) -> StorHba {
    let mut h = opened(server);
    start::run(&mut h).expect("the protocol starts");
    h
}

/// The driver up on `server` and its targets scanned.
pub fn up_on(server: Server) -> Up {
    let mut u = Units::new(started(server), REPLY, b"driver.storvsc0");
    u.scan(MAX_TARGET, MAX_LUN);
    u
}

pub fn up(units: Vec<((u8, u8), Unit)>) -> Up {
    up_on(Server::new(units))
}

pub fn seen(h: &StorHba) -> &[Seen] {
    &h.host.server.seen
}

pub fn ops(h: &StorHba) -> Vec<u32> {
    seen(h).iter().map(|s| s.op).collect()
}
