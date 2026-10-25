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

//! NCMP 0x0009 DISPLAY_HINT (section 12, Size): a guest service passing on
//! the host's wanted size. Body: output, width, height, source (1 VMMDev,
//! 2 VMware), u32 each; 0 x 0 is a probe. Taken only from a service named
//! in the kernel's HINT_SOURCES, empty until the owner accepts display
//! hints (Decision 15); everyone else is refused with -1.

pub const HINT_LEN: usize = 16;
pub const SOURCE_VMMDEV: u32 = 1;
pub const SOURCE_VMWARE: u32 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hint {
    pub output: u32,
    pub width: u32,
    pub height: u32,
    pub source: u32,
}

impl Hint {
    /// Exactly 16 bytes from a known source; anything else is refused.
    pub fn decode(b: &[u8]) -> Option<Hint> {
        if b.len() != HINT_LEN {
            return None;
        }
        let f = |i: usize| u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]);
        let h = Hint { output: f(0), width: f(4), height: f(8), source: f(12) };
        matches!(h.source, SOURCE_VMMDEV | SOURCE_VMWARE).then_some(h)
    }

    pub fn is_probe(&self) -> bool {
        self.width == 0 && self.height == 0
    }
}

/// Whether `sender` is one of the hint sources, given the pids their names
/// are announced under now (looked up per hint: a source that restarted
/// has a new pid, and a stale one must not keep the right).
pub fn allowed(sender: u32, sources: impl IntoIterator<Item = u32>) -> bool {
    sender != 0 && sources.into_iter().any(|pid| pid == sender)
}
