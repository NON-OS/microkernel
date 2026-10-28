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

//! A bound chip: its pipes, its link, the transfer it hands up, and counts.

use alloc::vec::Vec;

use nonos_usbnet::nic::ETH_FRAME_MAX;
use nonos_usbnet::xhci::BULK_MAX;
use nonos_usbnet::Pipes;

use super::watch::armed;
use super::{counts::Counts, function::AxFunction, rx_span::Span, tx_header::TX_HEADER};

/// One bulk IN: all driver.xhci0 moves at once, past the chip's (BULK_IN).
pub const RX_LEN: usize = BULK_MAX;
/// Frames taken from one transfer: 4 KiB holds 51 of 60 bytes at most.
pub const SPANS_MAX: usize = 64;

pub struct Ax<B> {
    pub(super) bus: B,
    pub(super) mac: [u8; 6],
    pub(super) pipes: Pipes,
    /// The status endpoint's DCI and what a look arms.
    pub(super) status: Option<u8>,
    pub(super) status_len: usize,
    /// Down from bind until the PHY reports a link; the medium last set.
    pub(super) link: bool,
    pub(super) medium: u16,
    pub(super) looks: u32,
    /// The rest of a transfer that overran `rx` is let go.
    pub(super) overrun: bool,
    pub(super) rx: Vec<u8>,
    /// The frames of the transfer in `rx` and the next to hand up.
    pub(super) spans: Vec<Span>,
    pub(super) next: usize,
    pub(super) tx: Vec<u8>,
    pub counts: Counts,
}

impl<B> Ax<B> {
    pub fn new(bus: B, mac: [u8; 6], f: &AxFunction, status: Option<u8>) -> Self {
        Self {
            bus,
            mac,
            pipes: f.pipes,
            status,
            status_len: armed(f.status.map_or(0, |n| n.max_packet)),
            link: false,
            medium: 0,
            looks: 0,
            overrun: false,
            rx: alloc::vec![0; RX_LEN],
            spans: Vec::with_capacity(SPANS_MAX),
            next: 0,
            tx: alloc::vec![0; TX_HEADER + ETH_FRAME_MAX + 1],
            counts: Counts::default(),
        }
    }
}
