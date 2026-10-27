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

//! The PHY told what to offer and reset into autonegotiation (IEEE 802.3
//! clause 22 registers, as recalled, unverified): 10 and 100 Mb/s half and
//! full duplex and symmetric and asymmetric pause in the advertisement,
//! 1000 Mb/s full duplex only in the 1000BASE-T control, then BMCR reset
//! with autonegotiation on and restarted. Linux v6.12 offers the same.
//! Its PHY tuning for the chip (EEE, analog trims, firmware patches) is
//! not done here: the PHY runs as its loaded settings leave it.

use super::bounds::PHY_RESET;
use super::op::{Op, Target};
use super::regs::phy::{ADVERTISE, BMCR, CTRL1000};

const TEN_HUNDRED: u32 = 0x0020 | 0x0040 | 0x0080 | 0x0100;
const PAUSE: u32 = 0x0400 | 0x0800;
const GIGA_HALF: u32 = 0x0100;
const GIGA_FULL: u32 = 0x0200;
const RESET: u32 = 0x8000;
const AN_ON: u32 = 0x1000;
const AN_RESTART: u32 = 0x0200;

pub const AUTONEG: [Op; 4] = [
    Op::Edit(Target::Phy(ADVERTISE), TEN_HUNDRED, TEN_HUNDRED | PAUSE),
    Op::Edit(Target::Phy(CTRL1000), GIGA_HALF | GIGA_FULL, GIGA_FULL),
    Op::Put(Target::Phy(BMCR), RESET | AN_ON | AN_RESTART),
    Op::Until(Target::Phy(BMCR), RESET, &[0], PHY_RESET),
];
