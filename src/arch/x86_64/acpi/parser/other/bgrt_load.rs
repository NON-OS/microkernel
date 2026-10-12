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

//! BGRT read at boot, and the logo it names copied out at once: firmware
//! leaves the image in boot services memory, which is RAM the kernel may
//! hand out later, so it is read here, early, or not at all. Nothing is
//! kept unless the copy is a BMP the splash can draw (`bmp_check`), the
//! header checked before the rest is read and the whole after.

use super::super::state::TableRegistry;
use super::bgrt_entry::{bgrt, Bgrt, IMAGE_BMP};
use super::bmp_check::{bmp, Bmp, FILE_HEADER};
use crate::arch::x86_64::acpi::tables::{SdtHeader, MAX_TABLE_BYTES};
use alloc::vec::Vec;
use spin::RwLock;

const SIG_BGRT: u32 = u32::from_le_bytes(*b"BGRT");
const MAX_IMAGE: u32 = 16 << 20;

static LOGO: RwLock<Option<(Bgrt, Bmp, Vec<u8>)>> = RwLock::new(None);

fn read(addr: u64, len: usize) -> Vec<u8> {
    // SAFETY: `addr` is the directmap view of RAM firmware named as the
    // image; `len` is bounded by MAX_IMAGE and the BMP's own size.
    (0..len).map(|i| unsafe { core::ptr::read_volatile((addr as *const u8).add(i)) }).collect()
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
    let Some(b) = bgrt(table).filter(|b| b.image_type == IMAGE_BMP) else { return };
    let Some(image) = super::super::phys::directmap(b.address) else { return };
    let head = bmp(&read(image, FILE_HEADER + 40), false);
    let Some(h) = head.filter(|h| h.file_size <= MAX_IMAGE) else {
        crate::log_warn!("[ACPI] BGRT image is not a BMP the splash can draw; not served");
        return;
    };
    let bytes = read(image, h.file_size as usize);
    if let Some(m) = bmp(&bytes, true).filter(|m| *m == h) {
        *LOGO.write() = Some((b, m, bytes));
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
