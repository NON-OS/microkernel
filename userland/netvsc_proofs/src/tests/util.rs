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

//! A host, the driver's channel opened on it, the way the capsule's setup
//! does it: the rings, then the start with its two buffers.

use nonos_vmbus::Pages;

use crate::constants::channel::{gpadl_recv, gpadl_rings, gpadl_send, OPEN_USER};
use crate::constants::layout::RINGS;
use crate::link::{Buffers, Link};
use crate::model::{Grants, NetHost, Netvsp, RECV_PFN, RINGS_PFN, SEND_PFN};
use crate::start::{self, Gpadls, Started};

pub const RELID: u32 = 17;
pub type NetLink = Link<NetHost, Pages>;

pub fn host() -> (Grants, Netvsp) {
    let g = Grants::new();
    (g, Netvsp::new(g))
}

/// The channel open on fresh grants, nothing more.
pub fn opened(g: Grants, netvsp: Netvsp) -> NetLink {
    let mut host = NetHost::new(netvsp);
    let mut rings = g.pages(RINGS_PFN);
    let chan = nonos_vmbus::open(&mut host, &mut rings, RELID, gpadl_rings(RELID), RINGS_PFN, RINGS, &OPEN_USER)
        .expect("the channel opens");
    let bufs = Buffers { recv: g.pages(RECV_PFN), send: g.pages(SEND_PFN), section: 0, sections: 0 };
    Link::new(host, rings, chan, bufs)
}

pub fn pfns(first: u64) -> [u64; 64] {
    core::array::from_fn(|i| first + i as u64)
}

pub fn start(l: &mut NetLink) -> Result<Started, &'static str> {
    let (recv, send) = (pfns(RECV_PFN), pfns(SEND_PFN));
    let g = Gpadls { relid: RELID, recv: (gpadl_recv(RELID), &recv), send: (gpadl_send(RELID), &send) };
    start::run(l, &g)
}

/// The driver up on a default host.
pub fn up() -> (NetLink, Started) {
    let (g, n) = host();
    let mut l = opened(g, n);
    let s = start(&mut l).expect("the start");
    assert!(l.host.netvsp.violations.is_empty(), "{:?}", l.host.netvsp.violations);
    (l, s)
}

/// Let the host run, then take what it sent: the host model runs when the
/// driver signals or waits, as Hyper-V's runs on its own.
pub fn pump(l: &mut NetLink) {
    use nonos_vmbus::Host;
    for _ in 0..3 {
        l.host.idle();
        l.service().unwrap();
    }
}

pub fn netvsp(l: &NetLink) -> &Netvsp {
    &l.host.netvsp
}

/// An Ethernet frame from `src` to `dst` of `len` bytes, its type IPv6 so
/// no rule of the wire's takes it for DHCP.
pub fn frame(dst: [u8; 6], src: [u8; 6], len: usize) -> Vec<u8> {
    let mut f = [&dst[..], &src[..], &[0x86, 0xDD]].concat();
    f.extend((0..len.saturating_sub(14)).map(|i| i as u8));
    f.truncate(len);
    f
}
