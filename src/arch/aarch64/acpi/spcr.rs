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

//! SPCR: the console the firmware used (layout recalled from Microsoft's
//! specification, unverified). 80 bytes through revision 2; revision 3
//! names the UART clock where 2 had reserved bytes; revision 4 adds the
//! precise baud rate and the console's namespace path.

use super::le::{u16_at, u32_at, u8_at};
use super::sdt::{revision, table, TableError};
pub use super::spcr_fields::Interface;
use super::spcr_fields::{baud, interface};
pub use super::spcr_tail::Gas;
use super::spcr_tail::{gas, namespace};
use alloc::vec::Vec;

const FIXED: usize = 80;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Spcr {
    pub interface: Interface,
    pub base: Gas,
    pub interrupt_type: u8,
    pub irq: u8,
    pub gsiv: u32,
    pub baud: Option<u32>,
    pub parity: u8,
    pub stop_bits: u8,
    pub flow_control: u8,
    /// Segment, bus, device, function; None when the port is no PCI device.
    pub pci: Option<(u8, u8, u8, u8)>,
    pub clock_hz: Option<u32>,
    pub precise_baud: Option<u32>,
    pub namespace: Option<Vec<u8>>,
}

pub fn spcr(t: &[u8]) -> Result<Spcr, TableError> {
    let t = table(t, b"SPCR")?;
    let (rev, s) = (revision(t), TableError::Short);
    if t.len() < FIXED || (rev >= 4 && t.len() < super::spcr_tail::FIXED_V4) {
        return Err(s);
    }
    let b = |at| u8_at(t, at).ok_or(s);
    let pci = match u16_at(t, 64).ok_or(s)? {
        0xFFFF => None,
        _ => Some((b(75)?, b(68)?, b(69)?, b(70)?)),
    };
    let namespace = if rev >= 4 { Some(namespace(t)?) } else { None };
    Ok(Spcr {
        interface: interface(b(36)?),
        base: gas(t, 40).ok_or(s)?,
        interrupt_type: b(52)?,
        irq: b(53)?,
        gsiv: u32_at(t, 54).ok_or(s)?,
        baud: baud(b(58)?),
        parity: b(59)?,
        stop_bits: b(60)?,
        flow_control: b(61)?,
        pci,
        clock_hz: if rev >= 3 { u32_at(t, 76).filter(|c| *c != 0) } else { None },
        precise_baud: if rev >= 4 { u32_at(t, 80).filter(|b| *b != 0) } else { None },
        namespace,
    })
}
