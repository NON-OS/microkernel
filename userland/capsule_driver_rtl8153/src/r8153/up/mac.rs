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

//! The station address: the one drawn for the run, written into PLA_IDR
//! as Linux set_ethernet_addr writes an address, with the config registers
//! unlocked (__rtl8152_set_mac_address), so the chip's unicast filter
//! (RCR_APM) matches it. Linux determine_ethernet_addr first reads the
//! factory address from PLA_BACKUP; that is never read here, so a chip
//! with none there binds all the same.

use nonos_usbnet::{Bus, Station};

use crate::r8153::fail::{at, Fail};
use crate::r8153::ocp::{set, write_byte, Dev, BYTE_EN_DWORD, BYTE_EN_WORD, PLA};
use crate::r8153::regs::bits::{CRWECR_CONFIG, CRWECR_NORMAL};
use crate::r8153::regs::pla::{CRWECR, IDR};

pub fn station_address<B: Bus>(dev: &mut Dev<B>, drawn: Station) -> Result<[u8; 6], Fail> {
    let mac = drawn.bytes();
    let mut raw = [0u8; 8];
    raw[..6].copy_from_slice(&mac);
    at("MAC not written to PLA_IDR", write_idr(dev, &raw))?;
    Ok(mac)
}

/// pla_ocp_write(PLA_IDR, BYTE_EN_SIX_BYTES, 8): generic_ocp_write sends
/// the first dword whole and the last with its low two bytes enabled.
fn write_idr<B: Bus>(dev: &mut Dev<B>, raw: &[u8; 8]) -> Result<(), i32> {
    write_byte(dev, PLA, CRWECR, CRWECR_CONFIG)?;
    set(dev, PLA | BYTE_EN_DWORD, IDR, &raw[..4])?;
    set(dev, PLA | BYTE_EN_WORD, IDR + 4, &raw[4..])?;
    write_byte(dev, PLA, CRWECR, CRWECR_NORMAL)
}
