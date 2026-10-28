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

//! The link, looked at on the frame service's clock (Nic::tick). While it
//! is down the PHY is read once every DOWN_EVERY requests, so a cable is
//! found within seconds. While it is up the status endpoint is read once
//! every LOOK_EVERY, and the PHY when a report says down, on one look in
//! SURE_EVERY besides (a report missed between looks), or on every look
//! for a chip with no status endpoint. A PHY read that fails counts toward
//! the device being taken as gone; a malformed report is only counted.

use nonos_usbnet::Bus;

use super::counts::bump;
use super::follow::follow;
use super::link::Ax;
use super::regs::PHY_STATUS;
use super::status::{link_hint, STATUS_LEN};
use super::vendor::phy_in;

/// Each look is one call to driver.xhci0; net.core asks every second.
pub const DOWN_EVERY: u32 = 2;
pub const LOOK_EVERY: u32 = 8;
pub const SURE_EVERY: u32 = 16;

/// What one look arms: one packet, never past a report.
pub fn armed(max_packet: u16) -> usize {
    (max_packet as usize).clamp(1, STATUS_LEN)
}

pub(super) fn watch<B: Bus>(ax: &mut Ax<B>) -> Result<(), i32> {
    ax.looks = ax.looks.wrapping_add(1);
    let every = if ax.link { LOOK_EVERY } else { DOWN_EVERY };
    if !ax.looks.is_multiple_of(every) {
        return Ok(());
    }
    let hint = report(ax);
    let sure = (ax.looks / LOOK_EVERY).is_multiple_of(SURE_EVERY);
    if ax.link && ax.status.is_some() && !sure && hint != Some(false) {
        return Ok(());
    }
    let phy = phy_in(&mut ax.bus, PHY_STATUS)?;
    follow(ax, phy)
}

fn report<B: Bus>(ax: &mut Ax<B>) -> Option<bool> {
    let dci = ax.status?;
    let mut n = [0u8; STATUS_LEN];
    let Ok(Some(len)) = ax.bus.interrupt_in_poll(dci, &mut n[..ax.status_len]) else {
        return None;
    };
    let hint = link_hint(&n[..len.min(ax.status_len)]);
    if hint.is_none() {
        bump(&mut ax.counts.notices, 1);
    }
    hint
}
