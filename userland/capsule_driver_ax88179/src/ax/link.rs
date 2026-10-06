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

//! A bound AX88179 as the frame service sees it.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::Deadline;
use nonos_usbnet::xhci::BULK_MAX;
use nonos_usbnet::{Bus, Nic, Pipes};

use super::link_poll::poll_link;
use super::transfer::{recv, send};
use super::tx::TX_MAX;

pub struct Ax88179<B> {
    pub(super) bus: B,
    pub(super) mac: [u8; 6],
    pub(super) pipes: Pipes,
    /// The last bulk IN, and the frames in it not yet handed up, as
    /// offset and length.
    pub(super) rx: Vec<u8>,
    pub(super) frames: Vec<(usize, usize)>,
    pub(super) next: usize,
    pub(super) tx: Vec<u8>,
    /// The speed and duplex bits of PHYSR the medium is set for; `None`
    /// while the link is down. Linux starts with the carrier off.
    pub(super) link: Option<u16>,
    pub(super) next_look: Deadline,
}

impl<B: Bus> Ax88179<B> {
    pub fn new(bus: B, mac: [u8; 6], pipes: Pipes) -> Self {
        let (rx, tx) = (vec![0; BULK_MAX], vec![0; TX_MAX]);
        let (frames, next, link) = (Vec::new(), 0, None);
        let next_look = Deadline::after_ms(0);
        Self { bus, mac, pipes, rx, frames, next, tx, link, next_look }
    }
}

impl<B: Bus> Nic for Ax88179<B> {
    fn mac(&self) -> [u8; 6] {
        self.mac
    }

    fn link_up(&self) -> bool {
        self.link.is_some()
    }

    fn send(&mut self, frame: &[u8]) -> Result<(), i32> {
        send(self, frame)
    }

    fn recv(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32> {
        recv(self, out)
    }

    fn tick(&mut self) {
        poll_link(self)
    }
}
