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

//! When the census is said, pure: `kernel_proofs` and `hwreport_proofs`
//! mount it. It starts looking once init has spawned everything and the
//! ACPI capsule's first publish pass is done (or there is no capsule to
//! wait for), so a host the capsule publishes late is in the table before
//! the census counts what the platform has; it is said at the first green
//! look, or `WAIT_MS` after init's spawns whatever it holds; then again on
//! any change.

/// How long after init's spawns a device may take to come up: a battery
/// whose EC starts late, a Wi-Fi part loading its firmware, a host the
/// ACPI capsule publishes late and the driver init then spawns for it.
pub const WAIT_MS: u64 = 30_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Look {
    /// The census was said before.
    pub said: bool,
    /// What it would say differs from what it said.
    pub changed: bool,
    pub green: bool,
    /// The ACPI capsule's first pass is done, or no capsule is coming.
    pub pass_done: bool,
    /// Since init's spawns.
    pub waited_ms: u64,
}

pub fn say_now(l: Look) -> bool {
    if l.said {
        return l.changed;
    }
    (l.green && l.pass_done) || l.waited_ms >= WAIT_MS
}

/// How much of `text` fits in `room` bytes, ending on a whole line.
pub fn whole_lines(text: &[u8], room: usize) -> usize {
    if text.len() <= room {
        return text.len();
    }
    text[..room].iter().rposition(|b| *b == b'\n').map_or(0, |at| at + 1)
}
