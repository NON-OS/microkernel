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

//! A bound NCM device as the frame service sees it: one frame per call
//! each way, while a received block's datagrams wait in `pending`.

use alloc::collections::VecDeque;
use alloc::vec;
use alloc::vec::Vec;

use nonos_usbnet::{Bus, Nic, Pipes};

use super::shape::TxShape;
use super::transfer::{recv, send};

pub struct Ncm<B> {
    pub(super) bus: B,
    pub(super) mac: [u8; 6],
    pub(super) pipes: Pipes,
    /// The NTB input size the device was given; one bulk IN is this long.
    pub rx_max: usize,
    pub shape: TxShape,
    pub(super) rx: Vec<u8>,
    pub(super) tx: Vec<u8>,
    pub(super) seq: u16,
    /// Each datagram of the last block, as its index and length in `rx`.
    pub(super) pending: VecDeque<(usize, usize)>,
}

impl<B: Bus> Ncm<B> {
    pub fn new(bus: B, mac: [u8; 6], pipes: Pipes, rx_max: usize, shape: TxShape) -> Self {
        let (rx, tx) = (vec![0; rx_max], vec![0; shape.max]);
        Self { bus, mac, pipes, rx_max, shape, rx, tx, seq: 0, pending: VecDeque::new() }
    }
}

impl<B: Bus> Nic for Ncm<B> {
    fn mac(&self) -> [u8; 6] {
        self.mac
    }

    /// The link is the device being bound: NETWORK_CONNECTION arrives on an
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
