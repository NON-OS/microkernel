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

use crate::arch::cpu::{disable_interrupts, enable_interrupts, interrupts_enabled};

/// # Safety
/// RAII guard that disables interrupts on creation and restores on drop.
/// Ensures interrupts are properly restored even on panic.
///
/// The mask, the restore and the enabled check all route through the
/// active arch backend (`<Arch as ArchOps>`), so the guard masks on
/// every target. On x86_64 that clears IF (CLI/STI); on aarch64 it
/// sets and clears the DAIF I bit. A no-op backend would leave the
/// guarded region running with interrupts live.
pub struct InterruptGuard {
    was_enabled: bool,
}

impl InterruptGuard {
    /// # Safety
    /// Creates guard by disabling interrupts if enabled.
    fn new() -> Self {
        let was_enabled = interrupts_enabled();
        if was_enabled {
            disable_interrupts();
        }
        Self { was_enabled }
    }
}

impl Drop for InterruptGuard {
    fn drop(&mut self) {
        if self.was_enabled {
            enable_interrupts();
        }
    }
}

/// # Safety
/// Creates RAII guard that disables interrupts until dropped.
pub fn disable_interrupts_guard() -> InterruptGuard {
    InterruptGuard::new()
}
