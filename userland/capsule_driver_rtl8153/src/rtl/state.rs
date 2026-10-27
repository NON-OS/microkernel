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

//! A bound chip: its pipes, its link, the transfer being walked, and what
//! it sent that was let go.

use alloc::vec;
use alloc::vec::Vec;

use nonos_usbnet::nic::ETH_FRAME_MAX;
use nonos_usbnet::Pipes;

use super::counts::Counts;
use super::model::Model;
use super::rx::RX_BUFFER;
use super::tx::TX_DESC;
use super::usb_speed::UsbSpeed;

pub struct Rtl<B> {
    pub(super) bus: B,
    pub(super) mac: [u8; 6],
    pub model: Model,
    pub(super) speed: UsbSpeed,
    /// The PHY register page last chosen (phy.rs).
    pub(super) page: Option<u16>,
    pub(super) pipes: Pipes,
    /// Down from bind until the link word says up and receive is on.
    pub(super) link: bool,
    pub(super) looks: u32,
    /// The last transfer in, its length, and where its walk stands.
    pub(super) rx: Vec<u8>,
    pub(super) have: usize,
    pub(super) at: usize,
    /// The rest of a transfer that overran the receive buffer is let go.
    pub(super) overrun: bool,
    pub(super) tx: Vec<u8>,
    pub counts: Counts,
}

impl<B> Rtl<B> {
    pub fn new(bus: B, mac: [u8; 6], model: Model, pipes: Pipes, page: Option<u16>) -> Self {
        Self {
            bus,
            mac,
            model,
            speed: super::usb_speed::usb_speed(pipes.max_packet_in),
            page,
            pipes,
            link: false,
            looks: 0,
            rx: vec![0; RX_BUFFER],
            have: 0,
            at: 0,
            overrun: false,
            tx: vec![0; TX_DESC + ETH_FRAME_MAX],
            counts: Counts::default(),
        }
    }
}
