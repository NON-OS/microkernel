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
//! rings, the receive buffer and the send buffer (grants.rs); the channel
//! opened on the rings; the protocol started and the station address
//! settled (finish.rs). Nothing is bound: the driver polls the host's ring.

use nonos_libc::mk_device_claim;
use nonos_vmbus::Pages;

use super::driver::{abandon, Driver};
use super::finish;
use super::grants::{self, pfns};
use crate::constants::channel::{gpadl_recv, gpadl_rings, gpadl_send, OPEN_USER};
use crate::constants::layout::{
    PAGE, RECV_BYTES, RECV_PAGES, RINGS, RING_BYTES, SEND_BYTES, SEND_PAGES,
};
use crate::discover::Found;
use crate::host::Kernel;
use crate::link::{Buffers, Link};
use crate::start::Gpadls;

pub fn run(dev: Found) -> Result<Driver, &'static str> {
    let epoch = mk_device_claim(dev.device_id);
    if epoch < 0 {
        return Err("claim failed");
    }
    let epoch = epoch as u64;
    let (grants, [r, rx, tx]) = grants::map_all(dev.device_id, epoch)?;
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
    let (first, gpadl) = (r.device_addr / PAGE as u64, gpadl_rings(relid));
    let chan =
        match nonos_vmbus::open(&mut host, &mut rings, relid, gpadl, first, RINGS, &OPEN_USER) {
            Ok(c) => c,
            Err(_) => {
                abandon(dev.device_id, &grants);
                return Err("channel did not open");
            }
        };
    let bufs = Buffers { recv, send, section: 0, sections: 0 };
    let link = Link::new(host, rings, chan, bufs);
    let (recv_pfns, send_pfns) = (pfns::<RECV_PAGES>(&rx), pfns::<SEND_PAGES>(&tx));
    let buffers = [gpadl_recv(relid), gpadl_send(relid)];
    let g = Gpadls { relid, recv: (buffers[0], &recv_pfns), send: (buffers[1], &send_pfns) };
    finish::finish(link, dev.device_id, grants, &g)
}
