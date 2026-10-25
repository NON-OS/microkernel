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

//! Whether the GOP copy may write the boot framebuffer: not while a claimed
//! device drives that memory (broker/table/boot_fb_rule.rs). Such a driver
//! may have re-moded its scanout, so the copy would land at the wrong pitch
//! or nowhere seen; the compositor moves to the driver on EBUSY, and GOP is
//! writable again once the claim goes. Asked on every present.

/// Whether a claimed device holds the boot framebuffer now.
pub(super) fn held() -> bool {
    let phys = crate::kernel_core::init::framebuffer::framebuffer_phys();
    phys.is_some_and(|(start, len)| crate::hardware::broker::boot_fb_held(start, len))
}
