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

//! A `BOX` entry: one Linux program's run in a box, as `capsule_linux`
//! enforced it, carrying BOX REPORT v1 field for field (pinned against
//! capsule_linux's sources by the proofs), without the examples:
//! `preset: u8 | policy: 32 | started: u8 | exit: i32 | counts: 9 x u64`.

use crate::{Reader, WireError};

/// The refusal kinds, in BOX REPORT v1's fixed order; a count's position
/// in `BoxRun::counts` is its kind.
pub const BOX_KINDS: [&str; 9] =
    ["write", "network", "exec", "jit", "display", "models", "call", "barred", "unserved"];

/// A `BOX` body is exactly this long.
pub const BOX_BODY: usize = 1 + 32 + 1 + 4 + 9 * 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    Strict,
    Offline,
    Allow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoxRun<'a> {
    pub preset: Preset,
    /// The BLAKE3 hash of the box's policy.
    pub policy: &'a [u8; 32],
    pub started: bool,
    pub exit: i32,
    pub counts: [u64; 9],
}

impl<'a> BoxRun<'a> {
    pub fn read(r: &mut Reader<'a>) -> Result<Self, WireError> {
        let preset = match r.u8()? {
            0 => Preset::Strict,
            1 => Preset::Offline,
            2 => Preset::Allow,
            _ => return Err(WireError::Value),
        };
        let policy = r.array()?;
        let started = match r.u8()? {
            0 => false,
            1 => true,
            _ => return Err(WireError::Value),
        };
        let exit = i32::from_le_bytes(*r.array()?);
        let mut counts = [0u64; 9];
        for c in counts.iter_mut() {
            *c = r.u64()?;
        }
        Ok(BoxRun { preset, policy, started, exit, counts })
    }

    /// Every refusal of the run: the counts' sum, held at `u64::MAX`.
    pub fn refused(&self) -> u64 {
        self.counts.iter().fold(0u64, |sum, c| sum.saturating_add(*c))
    }
}
