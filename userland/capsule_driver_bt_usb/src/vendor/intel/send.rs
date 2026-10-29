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

//! Intel Secure Send (0xfc09): one section of the file in fragments, each
//! its kind octet and at most 252 octets of the section, in order, each
//! answered before the next goes (`btintel_secure_send`).

use alloc::vec::Vec;

use super::wire::Wire;
use nonos_bt_hci::usb::UsbPipes;
use nonos_bt_hci::HciError;

pub const FRAGMENT_MAX: usize = 252;

/// The parameters of each Secure Send that carries `data` as `kind`.
pub fn pieces(kind: u8, data: &[u8]) -> impl Iterator<Item = Vec<u8>> + '_ {
    data.chunks(FRAGMENT_MAX).map(move |c| [&[kind][..], c].concat())
}

pub fn send<P: UsbPipes>(w: &mut Wire<P>, kind: u8, data: &[u8]) -> Result<(), HciError> {
    pieces(kind, data).try_for_each(|p| w.secure(&p))
}
