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

//! A register access by space: SystemMemory through a mapped window,
//! SystemIO and PCI_Config through the kernel. Every other space is refused.

use super::state::CapsuleHost;
use super::{memory_io, pci};
use nonos_aml::exec::RegionAccess;
use nonos_aml::region::Space;

pub(super) fn read(h: &mut CapsuleHost, a: &RegionAccess<'_>) -> Option<u64> {
    let v = match a.space {
        Space::SYSTEM_MEMORY => h
            .windows
            .va(a.address, a.width as u64, (a.base, a.len))
            .and_then(|va| memory_io::read(va, a.width)),
        Space::SYSTEM_IO if h.ports.ensure(a.address, a.width as u64, (a.base, a.len)) => {
            h.ports.read(a.address, a.width)
        }
        Space::PCI_CONFIG => a.pci.and_then(|p| pci::read(p, a.address, a.width)),
        _ => None,
    };
    h.refused += v.is_none() as u64;
    v
}
pub(super) fn write(h: &mut CapsuleHost, a: &RegionAccess<'_>, v: u64) -> bool {
    let done = match a.space {
        Space::SYSTEM_MEMORY => h
            .windows
            .va(a.address, a.width as u64, (a.base, a.len))
            .is_some_and(|va| memory_io::write(va, a.width, v)),
        Space::SYSTEM_IO => {
            h.ports.ensure(a.address, a.width as u64, (a.base, a.len))
                && h.ports.write(a.address, a.width, v)
        }
        Space::PCI_CONFIG => a.pci.is_some_and(|p| pci::write(p, a.address, a.width, v)),
        _ => false,
    };
    h.refused += !done as u64;
    done
}
