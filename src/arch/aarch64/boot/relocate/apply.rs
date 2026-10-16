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

use super::rela::{Placement, Refused, Rela, LOAD_ALIGN, R_AARCH64_RELATIVE};

/// Write every entry of `relas` into the image at `at.load_base`. Returns how
/// many were written.
///
/// # Safety
///
/// `[at.load_base, at.load_base + at.len)` must be the loaded image, writable,
/// and nothing may be reading it.
pub(crate) unsafe fn apply(relas: &[Rela], at: Placement) -> Result<usize, Refused> {
    let bias = at.load_base.wrapping_sub(at.link_base);
    if !bias.is_multiple_of(LOAD_ALIGN) {
        return Err(Refused::LoadMisaligned);
    }
    for r in relas {
        place(r, at)?;
    }
    for r in relas {
        // Wrapping only to keep panic paths out of code that runs before
        // relocation: `place` proved neither step can wrap.
        let at_place = at.load_base.wrapping_add(r.offset.wrapping_sub(at.link_base));
        // SAFETY: `place` checked this 8 bytes lies inside the image and is
        // aligned; the caller guarantees the image is writable and unread.
        unsafe { core::ptr::write_volatile(at_place as *mut u64, r.addend.wrapping_add(bias)) };
    }
    Ok(relas.len())
}

/// Checks one entry: RELATIVE, eight aligned bytes inside the image.
pub(super) fn place(r: &Rela, at: Placement) -> Result<(), Refused> {
    if r.info & 0xffff_ffff != R_AARCH64_RELATIVE {
        return Err(Refused::NotRelative);
    }
    let within = r.offset.checked_sub(at.link_base).ok_or(Refused::OutsideImage)?;
    if at.len < 8 || within > at.len.wrapping_sub(8) {
        return Err(Refused::OutsideImage);
    }
    if !r.offset.is_multiple_of(8) {
        return Err(Refused::MisalignedPlace);
    }
    Ok(())
}
