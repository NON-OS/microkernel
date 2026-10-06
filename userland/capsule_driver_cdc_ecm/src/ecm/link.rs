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

//! A bound ECM device as the frame service sees it.

use alloc::vec;
use alloc::vec::Vec;

use nonos_usbnet::nic::ETH_FRAME_MAX;
use nonos_usbnet::{Bus, Nic, Pipes};

use super::transfer::{recv, send};

/// One bulk IN: a whole frame and room for a padding byte, a multiple of
/// every bulk packet size, so a frame always ends in a short packet.
const RX_LEN: usize = 2048;

pub struct Ecm<B> {
    pub(super) bus: B,
    pub(super) mac: [u8; 6],
    pub(super) pipes: Pipes,
    pub(super) rx: Vec<u8>,
    pub(super) tx: Vec<u8>,
}

impl<B: Bus> Ecm<B> {
    pub fn new(bus: B, mac: [u8; 6], pipes: Pipes) -> Self {
        Self { bus, mac, pipes, rx: vec![0; RX_LEN], tx: vec![0; ETH_FRAME_MAX + 1] }
    }
}

impl<B: Bus> Nic for Ecm<B> {
    fn mac(&self) -> [u8; 6] {
        self.mac
    }

    /// The link is the device being bound: its notifications come on an
    /// interrupt endpoint driver.xhci0 does not configure beside bulk pipes.
    fn link_up(&self) -> bool {
        true
    }

    fn send(&mut self, frame: &[u8]) -> Result<(), i32> {
        send(self, frame)
    }

    fn recv(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32> {
        recv(self, out)
    }
}
