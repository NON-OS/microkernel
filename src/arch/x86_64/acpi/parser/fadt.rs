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

use alloc::vec::Vec;
use core::ptr;

use super::state::TableRegistry;
use crate::arch::x86_64::acpi::error::{AcpiError, AcpiResult};
use crate::arch::x86_64::acpi::hw::fadt_decode::{decode_fadt, FadtInfo};
use crate::arch::x86_64::acpi::tables::{PmProfile, SdtHeader, SIG_FADT};

/// No real FADT is anywhere near this; a larger length is a corrupt header
/// and only the first `MAX_FADT_BYTES` are looked at.
const MAX_FADT_BYTES: usize = 1024;

/// Copy the FADT out of firmware memory, bounded by its declared length.
/// Never reads past the table: a 116-byte ACPI 1.0 FADT is copied as 116
/// bytes and every later field decodes as zero.
fn copy_fadt(phys: u64) -> Option<Vec<u8>> {
    let virt = super::phys::directmap(phys)?;
    // SAFETY: `virt` maps the FADT the root table pointed at; the 36-byte
    // header is read first and bounds the rest of the copy.
    let header = unsafe { ptr::read_volatile(virt as *const SdtHeader) };
    let len = (header.length as usize).min(MAX_FADT_BYTES);
    if len < core::mem::size_of::<SdtHeader>() {
        return None;
    }
    let mut buf = Vec::new();
    buf.try_reserve_exact(len).ok()?;
    for i in 0..len {
        // SAFETY: inside the declared table length, see above.
        buf.push(unsafe { ptr::read_volatile((virt + i as u64) as *const u8) });
    }
    Some(buf)
}

pub fn parse_fadt(registry: &mut TableRegistry) -> AcpiResult<()> {
    let phys = *registry.tables.get(&SIG_FADT).ok_or(AcpiError::FadtNotFound)?;
    let bytes = copy_fadt(phys).ok_or(AcpiError::FadtNotFound)?;
    let info: FadtInfo = decode_fadt(&bytes).ok_or(AcpiError::FadtNotFound)?;

    if !info.checksum_ok {
        // Linux warns and carries on (acpi_tb_verify_checksum is advisory for
        // the FADT); refusing it would leave a laptop with no power control.
        crate::log_warn!("[ACPI] FADT checksum mismatch, using it anyway");
    }

    registry.data.pm_profile = PmProfile::from_u8(info.pm_profile);
    if info.sci_int != 0 || !info.is_hw_reduced() {
        registry.data.sci_interrupt = info.sci_int;
    }
    registry.data.has_8042 = info.has_8042();
    registry.data.fadt = Some(info);
    Ok(())
}
