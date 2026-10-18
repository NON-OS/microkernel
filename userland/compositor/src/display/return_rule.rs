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

//! Back to a display whose device came back. When a driver says its device
//! left, the desktop moves to the best other display announced
//! (setup/move_off.rs). The one it left is remembered by its place in the
//! kernel's list and the pid that said so. When that display's driver is
//! announced again under another pid, the desktop goes back to it, as long
//! as it still ranks above the display driven then. A display at least as
//! good, once driven, ends the memory.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Left {
    /// Its place in the display list, 0 best.
    pub rank: usize,
    /// The driver that said its device left; the same pid is not a return.
    pub pid: u32,
}

impl Left {
    /// Kept in one word: 0 nothing, else the rank plus one, then the pid.
    pub fn word(kept: Option<Left>) -> u64 {
        kept.map_or(0, |k| ((k.rank as u64 + 1) << 32) | u64::from(k.pid))
    }

    pub fn from_word(w: u64) -> Option<Left> {
        let r = w >> 32;
        (r != 0).then(|| Left { rank: (r - 1) as usize, pid: w as u32 })
    }
}

/// After leaving `leaving`: the better-ranked of it and what was kept. On a
/// tie the newer leaving, whose pid is the one that went last.
pub fn remember(kept: Option<Left>, leaving: Left) -> Option<Left> {
    Some(match kept {
        Some(k) if k.rank < leaving.rank => k,
        _ => leaving,
    })
}

/// What is still worth going back to while the display at `driving` is
/// driven: only one that ranks above it.
pub fn worth(kept: Option<Left>, driving: usize) -> Option<Left> {
    kept.filter(|k| k.rank < driving)
}

/// Whether to go back now: the kept display ranks above the one driven and
/// its driver is announced under a live pid that is not the one that left.
pub fn back(kept: Option<Left>, driving: usize, announced: Option<u32>) -> bool {
    match (worth(kept, driving), announced) {
        (Some(k), Some(pid)) => pid != 0 && pid != k.pid,
        _ => false,
    }
}
