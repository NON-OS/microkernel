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

//! The link as the device reports it, looked at on the frame service's
//! clock (Nic::tick): the notification endpoint read once every LOOK_EVERY
//! requests, NETWORK_CONNECTION kept, anything malformed counted.

use nonos_usbnet::Bus;

use super::counts::bump;
use super::link::Ecm;
use super::notice::{notice, Notice};

/// Each look is one call to driver.xhci0; the link changes seldom and
/// net.core asks for it once a second.
const LOOK_EVERY: u32 = 8;
/// The longest notification taken: a header and a speed change.
pub const NOTICE_MAX: usize = 16;

/// What one look arms: no more than one packet, so a notification that
/// fills its packet ends the transfer instead of waiting for the next one.
/// A speed change split over two packets is then let go and counted; its
/// rates are not used.
pub fn armed(max_packet: u16) -> usize {
    (max_packet as usize).clamp(1, NOTICE_MAX)
}

pub(super) fn watch<B: Bus>(ecm: &mut Ecm<B>) {
    ecm.looks = ecm.looks.wrapping_add(1);
    let Some(dci) = ecm.notify.filter(|_| ecm.looks.is_multiple_of(LOOK_EVERY)) else { return };
    let mut n = [0u8; NOTICE_MAX];
    let Ok(Some(len)) = ecm.bus.interrupt_in_poll(dci, &mut n[..ecm.notice_len]) else { return };
    match notice(&n[..len.min(ecm.notice_len)], ecm.comm) {
        Some(Notice::Link(up)) => ecm.link = up,
        Some(Notice::Speed | Notice::Other) => {}
        None => bump(&mut ecm.counts.notices),
    }
}
