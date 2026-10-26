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

//! Past the open channel: the protocol started (start.rs) and the station
//! address settled (station.rs); a failure closes the channel and gives
//! everything back.

use nonos_libc::{crypto_random, mk_debug};
use nonos_usbnet::station::draw;
use nonos_vmbus::Pages;

use super::driver::{abandon, Driver};
use crate::host::Kernel;
use crate::link::Link;
use crate::nic::Net;
use crate::probe::Verdict;
use crate::start::{self, Gpadls};
use crate::station;

/// The the maintainers' ruling, word for word.
const FELL_BACK: &[u8] =
    b"[NET] netvsc: Hyper-V drops a chosen MAC (MAC spoofing off); using the host-assigned MAC\n";

fn random_u32() -> Option<u32> {
    let mut b = [0u8; 4];
    (crypto_random(b.as_mut_ptr(), 4) == 4).then(|| u32::from_le_bytes(b))
}

pub fn finish(
    mut link: Link<Kernel, Pages>,
    device_id: u64,
    grants: [u64; 3],
    g: &Gpadls,
) -> Result<Driver, &'static str> {
    let (relid, buffers) = (g.relid, [g.recv.0, g.send.0]);
    let fail = |link: &mut Link<Kernel, Pages>, e: &'static str| {
        let _ = link.chan.close(&mut link.host);
        for id in buffers {
            let _ = nonos_vmbus::teardown(&mut link.host, relid, id);
        }
        abandon(device_id, &grants);
        e
    };
    let started = match start::run(&mut link, g) {
        Ok(s) => s,
        Err(e) => return Err(fail(&mut link, e)),
    };
    let (Some(drawn), Some(xid)) = (draw(), random_u32()) else {
        return Err(fail(&mut link, "no entropy for a station address"));
    };
    let drawn = drawn.bytes();
    let verdict = match station::choose(&mut link, &drawn, xid) {
        Ok(v) => v,
        Err(e) => return Err(fail(&mut link, e)),
    };
    if verdict != Verdict::Drawn {
        let _ = mk_debug(FELL_BACK.as_ptr(), FELL_BACK.len());
    }
    let mac = station::address(verdict, drawn, started.host_mac);
    let net = Net { link, mac, station: verdict.station(), started_up: started.link_up };
    Ok(Driver { net, device_id, grants, relid, buffers })
}
