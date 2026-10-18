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

//! A bound RNDIS device: its pipes, its limits, and the frames of the
//! last bulk IN.

use alloc::collections::VecDeque;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Range;

use nonos_usbnet::nic::ETH_FRAME_MAX;
use nonos_usbnet::xhci::BULK_MAX;
use nonos_usbnet::{Bus, Pipes, Taken};

use super::init::Limits;
use super::packet::PACKET_HDR;

/// One bulk IN, and the MaxTransferSize INITIALIZE gives the device: the
/// most driver.xhci0 moves at once, a multiple of every bulk packet size.
pub const RX_LEN: usize = BULK_MAX;

pub struct Rndis<B> {
    pub(super) bus: B,
    /// The drawn station address, and how the device took it.
    pub(super) station: ([u8; 6], Taken),
    pub(super) pipes: Pipes,
    pub(super) limits: Limits,
    pub(super) rx: Vec<u8>,
    /// Frames of the last bulk IN not yet handed up, as ranges of `rx`.
    pub(super) queue: VecDeque<Range<usize>>,
    pub(super) tx: Vec<u8>,
}

impl<B: Bus> Rndis<B> {
    pub fn new(bus: B, station: ([u8; 6], Taken), pipes: Pipes, limits: Limits) -> Self {
        let (rx, tx) = (vec![0; RX_LEN], vec![0; PACKET_HDR + ETH_FRAME_MAX + 1]);
        Self { bus, station, pipes, limits, rx, queue: VecDeque::new(), tx }
    }
}
