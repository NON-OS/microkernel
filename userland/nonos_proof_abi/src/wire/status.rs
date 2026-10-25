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

//! Reply statuses beyond the errnos (negative Linux values: EPERM -1, ENOENT
//! -2, EBUSY -16, EINVAL -22), and a job's stages.

/// The registry transcript's root is not one trust lists, or is past its life.
pub const STALE_REGISTRY: i32 = -1101;
/// plan/13 step 5 refused the policy: another app, or ids that do not match.
pub const POLICY: i32 = -1102;
/// This device's TPM keys are not in the registry transcript.
pub const NOT_ENROLLED: i32 = -1103;
/// A proof did not come out, or did not verify, or its certificate held not.
pub const PROVER: i32 = -1104;
/// The sealed run had a call outside the policy's set succeed: there is
/// nothing true to prove.
pub const OUTSIDE: i32 = -1105;
/// DELIVER's live record was not sealed under the vendor root (plan/15).
pub const NOT_VENDOR: i32 = -1106;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Stage {
    Queued = 0,
    Reading = 1,
    Device = 2,
    Isolation = 3,
    Done = 4,
    /// Ended without a container; STATUS's status says why.
    Failed = 5,
}

impl Stage {
    pub fn from_u8(v: u8) -> Option<Stage> {
        [Stage::Queued, Stage::Reading, Stage::Device, Stage::Isolation, Stage::Done, Stage::Failed]
            .get(usize::from(v))
            .copied()
    }
}

/// STATUS's reply body: the stage, then the container's length once Done.
pub fn status_body(stage: Stage, total: u32) -> [u8; 5] {
    let mut b = [0u8; 5];
    b[0] = stage as u8;
    b[1..].copy_from_slice(&total.to_le_bytes());
    b
}
