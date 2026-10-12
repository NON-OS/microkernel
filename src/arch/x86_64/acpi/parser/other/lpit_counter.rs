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

//! A low-power idle state's residency counter, read now. Read-only:
//! nothing here writes a register. A counter in system memory or I/O
//! space is read as any FADT register is. A functional fixed hardware one
//! is an MSR; it is read only when it is one of the package C-state
//! residency MSRs, so a firmware typo cannot fault the kernel. Those MSR
//! numbers are recalled from Intel's published register tables, not read:
//! unverified.

use super::lpit_load::lpit_state;
use crate::arch::x86_64::acpi::hw::gas::{gas_read, Gas, SPACE_SYSTEM_IO, SPACE_SYSTEM_MEMORY};
use crate::arch::x86_64::acpi::hw::port_bus::PortBus;

const SPACE_FFH: u8 = 0x7F;
/// PC2, PC3, PC6, PC7, PC8, PC9, PC10 residency.
const RESIDENCY_MSRS: [u64; 7] = [0x60D, 0x3F8, 0x3F9, 0x3FA, 0x630, 0x631, 0x632];

/// Why a counter gave no count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CounterError {
    NoState,
    NoCounter,
    Unreadable,
}

/// The `index`th state's residency counter.
pub fn lpit_residency(index: usize) -> Result<u64, CounterError> {
    let c =
        lpit_state(index).ok_or(CounterError::NoState)?.counter.ok_or(CounterError::NoCounter)?;
    match c.space {
        SPACE_SYSTEM_MEMORY | SPACE_SYSTEM_IO => {
            let g = Gas {
                space: c.space,
                bit_width: c.bit_width,
                bit_offset: c.bit_offset,
                access_size: c.access_size,
                address: c.address,
            };
            gas_read(&mut PortBus, &g).ok_or(CounterError::Unreadable)
        }
        SPACE_FFH if RESIDENCY_MSRS.contains(&c.address) => {
            let raw = crate::arch::x86_64::cpu::rdmsr(c.address as u32);
            let bits = raw.checked_shr(u32::from(c.bit_offset)).unwrap_or(0);
            Ok(match c.bit_width {
                1..=63 => bits & ((1 << c.bit_width) - 1),
                _ => bits,
            })
        }
        _ => Err(CounterError::Unreadable),
    }
}
