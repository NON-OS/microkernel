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

//! The DMAR table's flags byte. Bit 2, DMA_CTRL_PLATFORM_OPT_IN_FLAG, is
//! firmware saying it protected memory from DMA through boot and asks the OS
//! to keep it protected (VT-d 3.4, 8.1): the kernel then never lets
//! firmware's protection go except to its own tables.

use core::sync::atomic::{AtomicU8, Ordering};

const PLATFORM_OPT_IN: u8 = 1 << 2;

static FLAGS: AtomicU8 = AtomicU8::new(0);

pub(super) fn record(flags: u8) {
    FLAGS.store(flags, Ordering::Release);
}

/// Firmware set DMA_CTRL_PLATFORM_OPT_IN_FLAG; false with no DMAR table.
pub fn dmar_platform_opt_in() -> bool {
    FLAGS.load(Ordering::Acquire) & PLATFORM_OPT_IN != 0
}
