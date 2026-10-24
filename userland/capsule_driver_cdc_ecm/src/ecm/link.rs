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

//! A bound ECM device: its pipes, its link, and what it sent that was let go.

use alloc::vec;
use alloc::vec::Vec;

use nonos_usbnet::nic::ETH_FRAME_MAX;
use nonos_usbnet::{Pipes, Taken};

use super::counts::Counts;
use super::function::EcmFunction;
use super::watch::armed;

/// One bulk IN: a whole frame and room past it, a multiple of every bulk
/// packet size, so a frame of any allowed length ends in a short packet.
pub const RX_LEN: usize = 2048;

pub struct Ecm<B> {
    pub(super) bus: B,
    pub(super) mac: [u8; 6],
    /// How the device came to take frames for the drawn address.
    pub(super) taken: Taken,
    pub(super) comm: u8,
    pub(super) pipes: Pipes,
    /// The notification endpoint's DCI, and what one look at it arms.
    pub(super) notify: Option<u8>,
    pub(super) notice_len: usize,
    /// Up from bind until NETWORK_CONNECTION says down, so a device that
    /// never notifies still carries traffic.
    pub(super) link: bool,
    pub(super) looks: u32,
    /// The rest of a transfer that overran the receive buffer is let go.
    pub(super) overrun: bool,
    pub(super) rx: Vec<u8>,
    pub(super) tx: Vec<u8>,
    pub counts: Counts,
}

impl<B> Ecm<B> {
    pub fn new(bus: B, mac: [u8; 6], taken: Taken, f: &EcmFunction, notify: Option<u8>) -> Self {
        let notice_len = armed(f.notify.map_or(0, |n| n.max_packet));
        Self {
            bus,
            mac,
            taken,
            comm: f.comm,
            pipes: f.pipes,
            notify,
            notice_len,
            link: true,
            looks: 0,
            overrun: false,
            rx: vec![0; RX_LEN],
            tx: vec![0; ETH_FRAME_MAX + 1],
            counts: Counts::default(),
        }
    }
}
