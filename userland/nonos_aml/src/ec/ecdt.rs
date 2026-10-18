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

//! The Embedded Controller Boot Resources Table (ACPI 6.5, 5.2.16): the
//! EC's ports, GPE and namespace path, so its fields can be read before the
//! namespace has run the EC device's _REG.

use super::regs::EcPorts;
use crate::events::BlockError;
use alloc::vec::Vec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ecdt {
    pub ports: EcPorts,
    pub gpe: u8,
    /// The EC device's path, as the table spells it ("\_SB.PCI0.LPCB.EC0").
    pub path: Vec<u8>,
}

/// A GAS at `at` naming one System I/O port.
fn port(t: &[u8], at: usize) -> Result<u16, BlockError> {
    let g = t.get(at..at + 12).ok_or(BlockError::Range)?;
    let address = u64::from_le_bytes(g[4..12].try_into().map_err(|_| BlockError::Range)?);
    if g[0] != crate::events::SYSTEM_IO {
        return Err(BlockError::Memory);
    }
    u16::try_from(address).map_err(|_| BlockError::Range)
}

/// EC_CONTROL at 36, EC_DATA at 48, UID at 60, GPE_BIT at 64, EC_ID from 65.
pub fn parse_ecdt(t: &[u8]) -> Option<Ecdt> {
    if t.len() < 66 || t[..4] != *b"ECDT" {
        return None;
    }
    let ports = EcPorts { command: port(t, 36).ok()?, data: port(t, 48).ok()? };
    let path: Vec<u8> = t[65..].iter().copied().take_while(|b| *b != 0).collect();
    (!path.is_empty()).then_some(Ecdt { ports, gpe: t[64], path })
}
