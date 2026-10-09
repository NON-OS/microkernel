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

//! Config space of a function behind a VMD: an ECAM-shaped window in the
//! VMD's CFGBAR, relative to the domain's first bus.

use super::super::error::{PciError, Result};
use super::super::types::PciAddress;
use super::domain::cfg_offset;
use super::registry::find;

fn locate(address: PciAddress, offset: u16, width: u16) -> Result<u64> {
    let fail = PciError::ConfigAccessFailed {
        bus: address.bus,
        device: address.device,
        function: address.function,
        offset,
    };
    if offset & (width - 1) != 0 {
        return Err(PciError::UnalignedAccess { offset, alignment: width as u8 });
    }
    let domain = find(address.segment).ok_or(fail)?;
    let at = cfg_offset(
        domain.bus_start,
        domain.bus_count,
        address.bus,
        address.device,
        address.function,
        offset & !3,
    )
    .ok_or(fail)?;
    Ok(domain.cfg_va + at + (offset & 3) as u64)
}

macro_rules! access {
    ($read:ident, $write:ident, $ty:ty, $width:expr) => {
        pub fn $read(address: PciAddress, offset: u16) -> Result<$ty> {
            let va = locate(address, offset, $width)?;
            // SAFETY: eK@nonos.systems - `va` lies inside the CFGBAR mapping
            // the domain was registered with (cfg_offset bounds bus, device,
            // function and register), which is uncached device memory mapped
            // for the life of the kernel.
            Ok(unsafe { core::ptr::read_volatile(va as *const $ty) })
        }

        pub fn $write(address: PciAddress, offset: u16, value: $ty) -> Result<()> {
            let va = locate(address, offset, $width)?;
            // SAFETY: eK@nonos.systems - as the read: inside the mapped CFGBAR.
            unsafe { core::ptr::write_volatile(va as *mut $ty, value) };
            Ok(())
        }
    };
}

access!(read8, write8, u8, 1);
access!(read16, write16, u16, 2);
access!(read32, write32, u32, 4);
