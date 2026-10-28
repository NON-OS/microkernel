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

//! A host that never answers but never stops sending packets of its own: it
//! refills its ring each time the driver makes room, so the ring is never
//! empty. The wait still ends at its deadline on the clock, the reset's too.

use nonos_scsi::{Addr, Dir, Fault, Hba};

use super::flood_host::Flood;
use super::util::RELID;
use crate::constants::channel::{gpadl_id, OPEN_USER};
use crate::constants::layout::{DATA_OFF, GRANT_BYTES, PAGE, RINGS, RING_BYTES};
use crate::constants::timing::{COMMAND_MS, INIT_MS};
use crate::hba::VmHba;
use crate::model::{sim::Unit, Grant, Server, StorHost, BASE_PFN};

#[test]
fn a_host_that_floods_its_ring_cannot_hold_a_wait_past_its_deadline() {
    let grant = Grant::new(GRANT_BYTES as usize);
    let server = Server::new(vec![((0, 0), Unit::disk(2048, 512))]);
    let (mut host, mut rings) =
        (Flood(StorHost::new(grant, server), false, 0), grant.pages(RING_BYTES));
    let chan = nonos_vmbus::open(
        &mut host,
        &mut rings,
        RELID,
        gpadl_id(RELID),
        BASE_PFN,
        RINGS,
        &OPEN_USER,
    )
    .unwrap();
    // SAFETY: the grant's data buffer, leaked for the thread.
    let mut h = unsafe {
        VmHba::new(host, rings, chan, grant.va.add(DATA_OFF), BASE_PFN + (DATA_OFF / PAGE) as u64)
    };
    crate::start::run(&mut h).expect("the protocol starts");
    let s = &mut h.host.0.server;
    (s.silent, s.silent_resets, h.host.1) = (u32::MAX, true, true);
    let t0 = h.host.0.vmbus.now;
    assert_eq!(h.exec(Addr { target: 0, lun: 0 }, &[0; 6], Dir::None, 0), Err(Fault::Timeout));
    let took = h.host.0.vmbus.now - t0;
    assert!((COMMAND_MS + INIT_MS..COMMAND_MS + INIT_MS + 100).contains(&took), "{took} ms");
    assert!(h.dead, "the reset went unanswered too");
}
