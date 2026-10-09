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

use spin::Once;

use super::super::types::{flags, BootHandoffV1};

pub(crate) static BOOT_HANDOFF: Once<&'static BootHandoffV1> = Once::new();

#[inline]
pub fn get_handoff() -> Option<&'static BootHandoffV1> {
    BOOT_HANDOFF.get().copied()
}

#[inline]
pub fn is_initialized() -> bool {
    BOOT_HANDOFF.get().is_some()
}

pub fn total_memory() -> u64 {
    get_handoff().map(|h| unsafe { h.mmap.total_usable_memory() }).unwrap_or(0)
}

/*
 * Whether the boot menu's "Install NONOS" started this boot. False with no
 * handoff: a kernel that cannot read the request runs as it always does.
 */
pub fn install_requested() -> bool {
    get_handoff().is_some_and(|h| h.has_flag(flags::INSTALL_REQUESTED))
}
