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

//! Everything that decides how a sent NTB is laid out and how long it is,
//! fixed once at bind from the device's NTB parameters and its bulk OUT
//! packet size.

use nonos_usbnet::nic::ETH_FRAME_MAX;

use super::align::{out_align, OutAlign};
use super::limits::tx_max;
use super::ntb_out::layout;
use super::params::NtbParams;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TxShape {
    /// The longest block built, before its padding byte.
    pub max: usize,
    /// The device's dwNtbOutMaxSize: a block of that length needs no byte
    /// to end it.
    pub out_max: usize,
    pub mps: usize,
    pub align: OutAlign,
}

/// The shape for the device, or `None` when its largest block cannot carry
/// one full Ethernet frame: every frame the stack sends must go out.
pub fn tx_shape(p: &NtbParams, max_packet: u16) -> Option<TxShape> {
    let max = tx_max(p.out_max);
    let align = out_align(p, max);
    let s = TxShape { max, out_max: p.out_max as usize, mps: max_packet as usize, align };
    let (_, at) = layout(&s.align);
    (at + ETH_FRAME_MAX <= max).then_some(s)
}
