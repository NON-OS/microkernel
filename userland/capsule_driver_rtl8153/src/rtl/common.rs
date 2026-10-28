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

//! Runs of steps more than one phase takes.

use super::bounds::{MAC_RESET, PHY_ON, PHY_SETTLED};
use super::model::Model;
use super::op::{clear, put, until, Op, Target};
use super::reg::{pla16, pla8, usb16, usb32};
use super::regs::{phy, pla, usb};

/// The device takes no U1 or U2 link state: the first chip's six bytes of
/// U1/U2 tolerance zeroed, the B's LPM enable bit cleared.
pub fn no_u1u2(m: Model) -> &'static [Op] {
    const FIRST: [Op; 2] = [put(usb32(usb::U1U2), 0), put(usb16(usb::U1U2 + 4), 0)];
    const B: [Op; 1] = [clear(usb16(usb::LPM_CONFIG), 0x0001)];
    if m.is_b() {
        &B
    } else {
        &FIRST
    }
}

/// U2 power state 3 off.
pub const NO_U2P3: Op = clear(usb16(usb::U2P3), 0x0001);

/// The PHY out of power-down: wait for it to settle, take its power-down
/// bit off when set, then wait for LAN-on.
pub const PHY_UP: [Op; 3] = [
    Op::Until(Target::Phy(phy::STATE), 0x7, &[2, 3, 5], PHY_SETTLED),
    Op::EditIf(Target::Phy(phy::BMCR), 0x0800, Target::Phy(phy::BMCR), 0x0800, 0),
    Op::Until(Target::Phy(phy::STATE), 0x7, &[3], PHY_ON),
];

/// No runtime-suspend wake: the config registers unlocked, link-off wake
/// cleared, locked again.
pub const NO_SUSPEND_WAKE: [Op; 3] = [
    put(pla8(pla::CONFIG_LOCK), 0xc0),
    clear(pla16(pla::CONFIG34), 0x0008),
    put(pla8(pla::CONFIG_LOCK), 0x00),
];

/// No pending D3 and no link-change event left over.
pub const NO_D3: [Op; 3] = [
    clear(pla8(pla::D3_FLAG), 0x01),
    clear(pla8(pla::SUSPEND_FLAG), 0x01),
    clear(pla16(pla::EXTRA_STATUS), 0x0100),
];

/// The MAC's reset, which clears itself.
pub const MAC_RESET_RUN: [Op; 2] =
    [put(pla8(pla::COMMAND), 0x10), until(pla8(pla::COMMAND), 0x10, &[0], MAC_RESET)];

/// One frame per bulk IN: receive aggregation off (bit 4; Linux v6.12 does
/// the same for an adapter it knows mishandles it), and bit 7 cleared as
/// Linux clears it, its meaning undocumented.
pub const ONE_FRAME_IN: Op = Op::Edit(Target::Mac(usb16(usb::USB_CTRL)), 0x0080, 0x0010);
