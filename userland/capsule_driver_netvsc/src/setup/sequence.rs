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

//! The bring-up: the channel's record claimed; three DMA grants, for the
//! rings, the receive buffer and the send buffer; the channel opened on the
//! rings; the protocol started (start.rs); the station address settled
//! (station.rs). Nothing is bound: the driver polls the host's ring.

use nonos_libc::{crypto_random, mk_debug, mk_device_claim, mk_device_release, mk_dma_map, DmaMapOut, MK_DMA_MAP_COHERENT};
use nonos_usbnet::station::draw;
use nonos_vmbus::Pages;

use super::driver::{abandon, Driver};
use crate::constants::channel::{gpadl_recv, gpadl_rings, gpadl_send, OPEN_USER};
use crate::constants::layout::{PAGE, RECV_BYTES, RECV_PAGES, RINGS, RING_BYTES, SEND_BYTES, SEND_PAGES};
use crate::discover::Found;
use crate::host::Kernel;
use crate::link::{Buffers, Link};
use crate::nic::Net;
use crate::probe::Verdict;
use crate::start::{self, Gpadls};
use crate::station;

/// The the ruling, word for word.
const FELL_BACK: &[u8] =
    b"[NET] netvsc: Hyper-V drops a chosen MAC (MAC spoofing off); using the host-assigned MAC\n";

/// One coherent grant of `bytes` on the record: its address in the driver
/// and its first page number, as the host reaches it.
fn grant(dev: u64, epoch: u64, bytes: u64) -> Option<DmaMapOut> {
    let mut d = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    if mk_dma_map(dev, epoch, bytes, MK_DMA_MAP_COHERENT, &mut d) < 0 {
        return None;
    }
    if d.length < bytes || d.device_addr % PAGE as u64 != 0 {
        abandon_grant(d.grant_id);
        return None;
    }
    Some(d)
}

fn abandon_grant(id: u64) {
    let _ = nonos_libc::mk_dma_unmap(id);
}

fn pfns<const N: usize>(d: &DmaMapOut) -> [u64; N] {
    let first = d.device_addr / PAGE as u64;
    core::array::from_fn(|i| first + i as u64)
}

fn random_u32() -> Option<u32> {
    let mut b = [0u8; 4];
    (crypto_random(b.as_mut_ptr(), 4) == 4).then(|| u32::from_le_bytes(b))
}

pub fn run(dev: Found) -> Result<Driver, &'static str> {
    let epoch = mk_device_claim(dev.device_id);
    if epoch < 0 {
        return Err("claim failed");
    }
    let epoch = epoch as u64;
    let mut grants = [0u64; 3];
    let mut maps = [None, None, None];
    for (i, bytes) in [RING_BYTES, RECV_BYTES, SEND_BYTES].into_iter().enumerate() {
        match grant(dev.device_id, epoch, bytes) {
            Some(d) => {
                grants[i] = d.grant_id;
                maps[i] = Some(d);
            }
            None => {
                abandon(dev.device_id, &grants);
                return Err("dma map failed");
            }
        }
    }
    let [Some(r), Some(rx), Some(tx)] = maps else {
        let _ = mk_device_release(dev.device_id);
        return Err("dma map failed");
    };
    let relid = dev.relid;
    // SAFETY: each grant, mapped for the driver's life and reached by no one
    // else but the host.
    let (mut rings, recv, send) = unsafe {
        (
            Pages::new(r.user_va as *mut u8, RING_BYTES as usize),
            Pages::new(rx.user_va as *mut u8, RECV_BYTES as usize),
            Pages::new(tx.user_va as *mut u8, SEND_BYTES as usize),
        )
    };
    let mut host = Kernel::new(dev.device_id);
    let first = r.device_addr / PAGE as u64;
    let chan = match nonos_vmbus::open(&mut host, &mut rings, relid, gpadl_rings(relid), first, RINGS, &OPEN_USER) {
        Ok(c) => c,
        Err(_) => {
            abandon(dev.device_id, &grants);
            return Err("channel did not open");
        }
    };
    let bufs = Buffers { recv, send, section: 0, sections: 0 };
    let mut link = Link::new(host, rings, chan, bufs);
    let (recv_pfns, send_pfns) = (pfns::<RECV_PAGES>(&rx), pfns::<SEND_PAGES>(&tx));
    let buffers = [gpadl_recv(relid), gpadl_send(relid)];
    let g = Gpadls { relid, recv: (buffers[0], &recv_pfns), send: (buffers[1], &send_pfns) };
    let fail = |link: &mut Link<Kernel, Pages>, e: &'static str| {
        let _ = link.chan.close(&mut link.host);
        for id in buffers {
            let _ = nonos_vmbus::teardown(&mut link.host, relid, id);
        }
        abandon(dev.device_id, &grants);
        e
    };
    let started = match start::run(&mut link, &g) {
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
    let net = Net { link, mac, started_up: started.link_up };
    Ok(Driver { net, device_id: dev.device_id, grants, relid, buffers })
}
