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

//! Each input event's source, pure (K8b). The kernel writes it into bits 8
//! to 13 of `flags`, over whatever the poster put there, so input_router
//! knows which device typed and no poster can claim to be another. It is a
//! small index a poster gets on its first post, never a pid; input_router
//! clears it before an event reaches an app. Every modifier bit in use is in
//! bits 0 to 6 (app_skeleton input/modifiers.rs); bits 14 and 15 stay the
//! poster's, the Bluetooth capsule's bond marks (input_router route/bond.rs).
//! Proven in kernel_proofs::input_source.

/// The kernel's own events: the power button, the probe injector.
pub(crate) const KERNEL: u8 = 0;
/// A poster past the table's last slot.
pub(crate) const UNREGISTERED: u8 = 63;
/// Slots for posting processes; slot `i` is source `i + 1`.
pub(crate) const SLOTS: usize = 62;
/// The bits of `flags` a source takes.
pub(crate) const SOURCE_BITS: u16 = 0x3F00;

/// `flags` with bits 8 to 13 replaced by `source`, the rest kept.
pub(crate) const fn stamp(flags: u16, source: u8) -> u16 {
    (flags & !SOURCE_BITS) | ((source as u16 & 0x3F) << 8)
}

/// The source of the process `poster`, registering it in the first free
/// slot, or the first whose process `alive` says has gone.
pub(crate) fn index_of(table: &mut [u32; SLOTS], poster: u32, alive: impl Fn(u32) -> bool) -> u8 {
    if poster == 0 {
        return UNREGISTERED;
    }
    if let Some(i) = table.iter().position(|&p| p == poster) {
        return (i + 1) as u8;
    }
    match table.iter().position(|&p| p == 0 || !alive(p)) {
        Some(i) => {
            table[i] = poster;
            (i + 1) as u8
        }
        None => UNREGISTERED,
    }
}
