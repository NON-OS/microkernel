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

//! What a check is, and what it can say.

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Verdict {
    /// Checked, and the claim survived.
    Holds,
    /// Checked, and it did not. This is the state that matters: the screen exists
    /// so that this can appear.
    Broken,
    /// Not established. The kernel did not answer, so nothing is claimed either
    /// way; an unanswered question must never render as a pass.
    Unknown,
}

impl Verdict {
    /// A settled yes-or-no.
    pub fn from_bool(ok: bool) -> Self {
        if ok {
            Verdict::Holds
        } else {
            Verdict::Broken
        }
    }
}

/// What the bootloader measured before the kernel had control. Nothing in here
/// is re-verified by the act of reading it.
pub struct Recorded {
    pub kernel_signature: Verdict,
    pub secure_boot: Verdict,
    pub attestation: Verdict,
    pub proof: Verdict,
    pub kernel_hash: [u8; 32],
    pub program_hash: [u8; 32],
}

/// One claim tested against the live kernel, with the count that settled it so
/// the reader can see the arithmetic rather than trust the tick.
pub struct Check {
    pub claim: &'static [u8],
    pub verdict: Verdict,
    /// How many processes matched the condition the claim forbids.
    pub found: u32,
    pub of: u32,
}

/// Counts that are true but untestable. Nothing here can be wrong, so nothing
/// here gets a verdict.
pub struct Census {
    pub capsules: u32,
    pub filesystem: u32,
    pub raw_hardware: u32,
    pub own_mask: u64,
}

pub struct Live {
    pub checks: [Check; 4],
    pub census: Census,
}
