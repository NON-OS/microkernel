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

use super::super::hardware::{has_cpu_entropy, has_cpu_random};
use super::super::state::BOOTLOADER_ENTROPY_PROVIDED;
use core::sync::atomic::Ordering;

/// One boot line naming every source the RNG was seeded from, so a board
/// whose only entropy is a firmware seed says so rather than leaving it to
/// be guessed from what it lacks. The entropy capsule's bytes come later
/// and are never enough alone, so they are not one of these.
pub fn report_entropy_sources() {
    let sources: [(bool, &[u8]); 3] = [
        (has_cpu_entropy(), b" cpu-entropy-instruction"),
        (has_cpu_random(), b" cpu-random-instruction"),
        (BOOTLOADER_ENTROPY_PROVIDED.load(Ordering::Acquire), b" firmware-seed"),
    ];
    let mut line = crate::sys::serial::Line::new();
    line.str(b"[RNG] seeded from:");
    for (_, name) in sources.iter().filter(|(present, _)| *present) {
        line.str(name);
    }
    if sources.iter().all(|(present, _)| !present) {
        line.str(b" no source a device vouches for");
    }
    line.end();
}
