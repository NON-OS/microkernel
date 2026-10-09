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

//! A bound RTL8153: the chip, what bring-up learned of it, the link as
//! last read, and the transfer buffers.

use alloc::collections::VecDeque;
use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::Deadline;
use nonos_usbnet::nic::ETH_FRAME_MAX;
use nonos_usbnet::xhci::BULK_MAX;
use nonos_usbnet::Pipes;

use super::ocp::Dev;
use super::tx::TX_DESC;
use super::Version;

pub struct Rtl8153<B> {
    pub(super) dev: Dev<B>,
    pub(super) version: Version,
    pub(super) mac: [u8; 6],
    pub(super) pipes: Pipes,
    /// PLA_PHYSTATUS's LINK_STATUS when last read, with traffic started.
    pub(super) link: bool,
    pub(super) next_look: Deadline,
    /// One bulk IN, as large as driver.xhci0 moves.
    pub(super) rx: Vec<u8>,
    pub(super) tx: Vec<u8>,
    /// Frames of the last bulk IN not yet handed up.
    pub(super) queue: VecDeque<Vec<u8>>,
    /// Where the link's changes are told; main.rs points it at the log.
    pub note: fn(&[&[u8]]),
}

impl<B> Rtl8153<B> {
    pub fn new(dev: Dev<B>, version: Version, mac: [u8; 6], pipes: Pipes) -> Self {
        Self {
            dev,
            version,
            mac,
            pipes,
            link: false,
            next_look: Deadline::after_ms(0),
            rx: vec![0; BULK_MAX],
            tx: vec![0; TX_DESC + ETH_FRAME_MAX],
            queue: VecDeque::new(),
            note: |_| {},
        }
    }

    pub fn version(&self) -> Version {
        self.version
    }
}
