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

//! BGRT read at boot, and the logo it names copied out at once
//! (`bgrt_copy`): firmware leaves the image in boot services memory, which
//! is RAM the kernel may hand out later, so it is read here, early, or not
//! at all.

use super::super::state::TableRegistry;
use super::bgrt_copy::{copy_logo, Logo, LogoError};
use super::bgrt_entry::Bgrt;
use super::bmp_check::Bmp;
use crate::arch::x86_64::acpi::tables::{SdtHeader, MAX_TABLE_BYTES};
use alloc::vec::Vec;
use spin::RwLock;

const SIG_BGRT: u32 = u32::from_le_bytes(*b"BGRT");

static LOGO: RwLock<Option<Logo>> = RwLock::new(None);

fn read(phys: u64, len: usize) -> Option<Vec<u8>> {
    let addr = super::super::phys::directmap(phys)?;
    // SAFETY: `addr` is the directmap view of RAM firmware named as the
    // image; `len` is bounded by the BMP's headers and MAX_IMAGE.
    Some(
        (0..len).map(|i| unsafe { core::ptr::read_volatile((addr as *const u8).add(i)) }).collect(),
    )
}

pub fn parse_bgrt(registry: &mut TableRegistry) {
    let Some(&phys) = registry.tables.get(&SIG_BGRT) else { return };
    let Some(addr) = super::super::phys::directmap(phys) else { return };
    // SAFETY: as for SRAT and HMAT: the directmap covers what firmware
    // published, and the length is trusted only once the checksum holds.
    let table = unsafe {
        let header = core::ptr::read_volatile(addr as *const SdtHeader);
        if !header.validate_checksum(addr as *const u8) {
            return;
        }
        let len = u64::from(header.length).min(MAX_TABLE_BYTES) as usize;
        core::slice::from_raw_parts(addr as *const u8, len)
    };
    match copy_logo(table, read) {
        Ok(logo) => {
            *LOGO.write() = Some(logo);
            super::bgrt_mode::record();
        }
        Err(LogoError::NoLogo) => {}
        Err(e) => crate::log_warn!("[ACPI] BGRT logo not served: {:?}", e),
    }
}

/// Where the logo is and what it is.
pub fn boot_logo() -> Option<(Bgrt, Bmp)> {
    LOGO.read().as_ref().map(|(b, m, _)| (*b, *m))
}

/// The logo's BMP file, as copied at boot.
pub fn boot_logo_file<R>(f: impl FnOnce(&[u8]) -> R) -> Option<R> {
    LOGO.read().as_ref().map(|(_, _, bytes)| f(bytes))
}
