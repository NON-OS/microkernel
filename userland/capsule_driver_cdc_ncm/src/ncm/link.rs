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

//! A bound NCM device: its station address, pipes and NTB sizes, and a
//! received block's datagrams waiting in `pending`, one handed up per call.

use alloc::collections::VecDeque;
use alloc::vec;
use alloc::vec::Vec;

use nonos_usbnet::{Bus, Pipes, Taken};

use super::shape::TxShape;

pub struct Ncm<B> {
    pub(super) bus: B,
    /// The drawn station address, and how the device took it.
    pub(super) station: ([u8; 6], Taken),
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
    pub fn new(
        bus: B,
        station: ([u8; 6], Taken),
        pipes: Pipes,
        rx_max: usize,
        shape: TxShape,
    ) -> Self {
        let (rx, tx) = (vec![0; rx_max], vec![0; shape.max]);
        Self { bus, station, pipes, rx_max, shape, rx, tx, seq: 0, pending: VecDeque::new() }
    }
}
