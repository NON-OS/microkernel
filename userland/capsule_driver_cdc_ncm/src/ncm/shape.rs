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

use super::align::{out_align, OutAlign};
use super::limits::{min_tx_pkt, tx_max};
use super::params::NtbParams;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TxShape {
    /// The largest NTB sent, and the length a padded one is given.
    pub max: usize,
    /// A block longer than this is padded to `max`.
    pub min_pkt: usize,
    pub mps: usize,
    /// The device is one Linux sends whole-size blocks to without padding
    /// (FLAG_SEND_ZLP).
    pub zlp: bool,
    pub align: OutAlign,
}

pub fn tx_shape(p: &NtbParams, max_packet: u16, zlp: bool) -> TxShape {
    let mps = max_packet as usize;
    let max = tx_max(p.out_max, mps);
    let align = out_align(p, max);
    TxShape { max, min_pkt: min_tx_pkt(max, mps), mps, zlp, align }
}
