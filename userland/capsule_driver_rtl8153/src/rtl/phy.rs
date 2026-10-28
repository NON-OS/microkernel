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

//! The PHY's registers, a word each: the PLA register at 0xe86c chooses
//! which 4 KiB page of them the PLA window at 0xb000 shows. The page last
//! chosen is kept, and chosen again only when it changes or is not known
//! (a failed choice leaves it unknown).

use nonos_usbnet::Bus;

use super::ocp::{read, write};
use super::reg::pla16;
use super::regs::pla::PHY_PAGE;

const WINDOW: u16 = 0xb000;

fn choose<B: Bus>(bus: &mut B, page: &mut Option<u16>, addr: u16) -> Result<u16, i32> {
    let want = addr & 0xf000;
    if *page != Some(want) {
        *page = None;
        write(bus, pla16(PHY_PAGE), want as u32)?;
        *page = Some(want);
    }
    Ok(WINDOW | (addr & 0x0fff))
}

pub fn phy_read<B: Bus>(bus: &mut B, page: &mut Option<u16>, addr: u16) -> Result<u32, i32> {
    let at = choose(bus, page, addr)?;
    read(bus, pla16(at))
}

pub fn phy_write<B: Bus>(
    bus: &mut B,
    page: &mut Option<u16>,
    addr: u16,
    v: u32,
) -> Result<(), i32> {
    let at = choose(bus, page, addr)?;
    write(bus, pla16(at), v)
}
