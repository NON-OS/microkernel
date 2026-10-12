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

//! The link as the device reports it: its notification endpoint looked
//! at now and then from the receive path, NETWORK_CONNECTION kept.

use nonos_usbnet::Bus;

use super::link::Ncm;
use super::notice::connected;

/// The notification endpoint looked at once every LOOK_EVERY receives: the
/// link changes seldom, and each look is one call to the host.
const LOOK_EVERY: u32 = 64;

pub(super) fn look_at_link<B: Bus>(ncm: &mut Ncm<B>) {
    ncm.looks = ncm.looks.wrapping_add(1);
    let Some(dci) = ncm.notify.filter(|_| ncm.looks.is_multiple_of(LOOK_EVERY)) else { return };
    let mut n = [0u8; 16];
    if let Ok(Some(len)) = ncm.bus.interrupt_in_poll(dci, &mut n) {
        ncm.link = connected(&n[..len]).unwrap_or(ncm.link);
    }
}
