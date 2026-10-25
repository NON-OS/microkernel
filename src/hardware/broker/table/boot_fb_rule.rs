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
/* Whether a claimed device holds the boot framebuffer, pure. A device whose
 * memory window contains the framebuffer the loader handed over (the GPU
 * behind GOP, or bootfb serving that window) drives that memory itself once
 * claimed: it may re-mode its scanout, so the kernel's copy of a GOP frame
 * there lands at the wrong pitch or nowhere seen. While one holds it,
 * MkSurfacePresent on GOP answers EBUSY and the compositor moves to the
 * driver; when the claim goes, GOP is writable again. Asked afresh on each
 * present, never remembered. kernel_proofs::boot_fb. */

/// A memory window's kind, as the device record carries it (BAR_KIND_MMIO).
pub(crate) const MMIO: u8 = 1;

/// Whether `[start, start + len)` lies wholly inside the memory window
/// `(kind, base, size)`.
pub(crate) fn contains(window: (u8, u64, u64), start: u64, len: u64) -> bool {
    let (kind, base, size) = window;
    let (Some(end), Some(top)) = (start.checked_add(len), base.checked_add(size)) else {
        return false;
    };
    kind == MMIO && len > 0 && size > 0 && base <= start && end <= top
}

/// Whether any claimed device has a window, any of its windows, that
/// contains the boot framebuffer `(start, len)`. `claimed` gives each
/// claimed device's windows; `window` reads one as (kind, base, size).
pub(crate) fn holds_boot_fb<D, W>(
    fb: (u64, u64),
    claimed: impl IntoIterator<Item = D>,
    window: impl Fn(&W) -> (u8, u64, u64),
) -> bool
where
    D: IntoIterator<Item = W>,
{
    claimed.into_iter().any(|wins| wins.into_iter().any(|w| contains(window(&w), fb.0, fb.1)))
}
