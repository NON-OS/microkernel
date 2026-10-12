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
//! the answer kept for link_up. Each failure goes to the run loop, which
//! counts it toward taking the device as gone.

use nonos_libc::Deadline;
use nonos_usbnet::Bus;

use super::access::read_phy;
use super::link::Ax88179;
use super::link_reset::link_reset;
use super::phy_regs::{PHYSR, PHYSR_FULL, PHYSR_LINK, PHYSR_SMASK};

pub(super) const LINK_POLL_MS: u64 = 1_000;
/// PHYSR reads failed in a row before the last reading is no longer
/// believed and the link reads down.
const UNREAD_MAX: u8 = 3;
/// How long a link reset that failed or did not settle waits before it
/// runs again, so a chip that never settles costs its transfers at most
/// once in this long, not once a second.
const RESET_AGAIN_MS: u64 = 5_000;

/// The bits a medium mode depends on: speed and duplex.
const RESOLVED: u16 = PHYSR_SMASK | PHYSR_FULL;

pub(super) fn poll_link<B: Bus>(ax: &mut Ax88179<B>) -> Result<(), i32> {
    if !ax.next_look.expired() {
        return Ok(());
    }
    ax.next_look = Deadline::after_ms(LINK_POLL_MS);
    let physr = match read_phy(&mut ax.bus, PHYSR, "PHY PHYSR unread") {
        Ok(physr) => physr,
        Err((_, e)) => {
            ax.unread = ax.unread.saturating_add(1);
            if ax.unread >= UNREAD_MAX {
                ax.link = None;
            }
            return Err(e);
        }
    };
    ax.unread = 0;
    if physr & PHYSR_LINK == 0 {
        ax.link = None;
        return Ok(());
    }
    // A new link, or one the partner renegotiated, gets its medium as
    // Linux runs ax88179_link_reset on each change.
    if ax.link == Some(physr & RESOLVED) || !ax.next_reset.expired() {
        return Ok(());
    }
    let reset = link_reset(&mut ax.bus);
    ax.link = reset.ok().flatten().map(|p| p & RESOLVED);
    if ax.link.is_none() {
        ax.next_reset = Deadline::after_ms(RESET_AGAIN_MS);
    }
    reset.map(|_| ()).map_err(|(_, e)| e)
}
