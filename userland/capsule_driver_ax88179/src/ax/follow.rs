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

//! The MAC set to the link the PHY reports (as Linux v6.12's link reset
//! does it, read for behaviour): on a link, or a new speed or duplex, the
//! receiver restarted until the chip's send FIFO reads drained (at most
//! RESTARTS rounds, never a wait), the bulk-in queue set again and the
//! medium written; the link is up only once all of it was taken. Without
//! a link the link is down and nothing is written.

use nonos_usbnet::Bus;

use super::link::Ax;
use super::medium::{link_of, medium_for};
use super::plan::BULK_IN;
use super::regs::{BULK_IN_QUEUE, MEDIUM, RX_CONTROL, RX_OFF, RX_ON};
use super::vendor::{mac_out, vendor_in};

/// The rounds of receiver restart one link change may take.
pub const RESTARTS: usize = 4;
/// The FIFO status read: vendor request 81h, wValue 8Ch, four bytes; bit 30
/// set while the FIFO holds data (its meaning as Linux names it, unverified).
const FIFO_READ: u8 = 0x81;
const FIFO_AT: (u16, u16) = (0x8C, 0);
const FIFO_BUSY: u32 = 1 << 30;

pub(super) fn follow<B: Bus>(ax: &mut Ax<B>, phy: u16) -> Result<(), i32> {
    if !link_of(phy) {
        ax.link = false;
        return Ok(());
    }
    let medium = medium_for(phy);
    if ax.link && ax.medium == medium {
        return Ok(());
    }
    ax.link = false;
    restart_rx(&mut ax.bus)?;
    mac_out(&mut ax.bus, BULK_IN_QUEUE, &BULK_IN)?;
    mac_out(&mut ax.bus, MEDIUM, &medium.to_le_bytes())?;
    (ax.link, ax.medium) = (true, medium);
    Ok(())
}

fn restart_rx<B: Bus>(bus: &mut B) -> Result<(), i32> {
    for _ in 0..RESTARTS {
        mac_out(bus, RX_CONTROL, &RX_OFF.to_le_bytes())?;
        mac_out(bus, RX_CONTROL, &RX_ON.to_le_bytes())?;
        let mut w = [0u8; 4];
        // A chip that does not answer the read has nothing to wait for.
        if vendor_in(bus, FIFO_READ, FIFO_AT, &mut w).is_err() {
            break;
        }
        if u32::from_le_bytes(w) & FIFO_BUSY == 0 {
            break;
        }
    }
    Ok(())
}
