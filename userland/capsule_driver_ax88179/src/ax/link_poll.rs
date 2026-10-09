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

//! The link, read on the clock. Linux learns of a change from the chip's
//! interrupt endpoint (ax88179_status); driver.xhci0 configures none
//! beside the bulk pipes, so the PHY is read at most once a second and
//! the answer kept for link_up.

use nonos_libc::Deadline;
use nonos_usbnet::Bus;

use super::access::read_phy;
use super::link::Ax88179;
use super::link_reset::link_reset;
use super::phy_regs::{PHYSR, PHYSR_FULL, PHYSR_LINK, PHYSR_SMASK};

pub(super) const LINK_POLL_MS: u64 = 1_000;

/// The bits a medium mode depends on: speed and duplex.
const RESOLVED: u16 = PHYSR_SMASK | PHYSR_FULL;

pub(super) fn poll_link<B: Bus>(ax: &mut Ax88179<B>) {
    if !ax.next_look.expired() {
        return;
    }
    ax.next_look = Deadline::after_ms(LINK_POLL_MS);
    // A PHY that does not answer leaves the last reading: the transfers
    // that fail with it are what tells the run loop the device has gone.
    let Ok(physr) = read_phy(&mut ax.bus, PHYSR, "PHY PHYSR unread") else { return };
    if physr & PHYSR_LINK == 0 {
        ax.link = None;
        return;
    }
    // A new link, or one the partner renegotiated, gets its medium as
    // Linux runs ax88179_link_reset on each change.
    if ax.link != Some(physr & RESOLVED) {
        ax.link = link_reset(&mut ax.bus).ok().flatten().map(|p| p & RESOLVED);
    }
}
