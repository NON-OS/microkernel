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

use nonos_usbnet::{Bus, Pipes};

use super::shape::TxShape;

pub struct Ncm<B> {
    pub(super) bus: B,
    pub(super) mac: [u8; 6],
    pub(super) pipes: Pipes,
    /// The NTB input size the device was given; one bulk IN is this long.
    pub rx_max: usize,
    /// The device offered 32-bit NTBs, so one it sends is read.
    pub ntb32: bool,
    pub shape: TxShape,
    pub(super) rx: Vec<u8>,
    pub(super) tx: Vec<u8>,
    pub(super) seq: u16,
    /// Each datagram of the last block, as its index and length in `rx`.
    pub(super) pending: VecDeque<(usize, usize)>,
    /// The notification endpoint's DCI, and the link it last reported.
    pub notify: Option<u8>,
    pub(super) link: bool,
    pub(super) looks: u32,
}

impl<B: Bus> Ncm<B> {
    pub fn new(bus: B, mac: [u8; 6], pipes: Pipes, rx_max: usize, shape: TxShape) -> Self {
        let (rx, tx) = (vec![0; rx_max], vec![0; shape.max]);
        let pending = VecDeque::new();
        let (notify, link, looks, ntb32) = (None, true, 0, false);
        Self { bus, mac, pipes, rx_max, ntb32, shape, rx, tx, seq: 0, pending, notify, link, looks }
    }
}
