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

//! The `MkBootAttest` layout alone, with no imports, for a host test to include.
//!
//! ```text
//! 0       version
//! 1       state: NOT_YET, MEASURED, SELF_REPORTED, REFUSED, NO_EVIDENCE
//! 2..4    zero
//! 4..8    refusal code, u32 little-endian; zero unless REFUSED
//! 8..16   epoch, u64 little-endian
//! 16..48  the loader's measurement, its Authenticode SHA-256
//! 48..80  the boot root it is enrolled under
//! ```

pub const RECORD_VERSION: u8 = 1;
pub const RECORD_LEN: usize = 80;
pub const NOT_YET: u8 = 0;
pub const MEASURED: u8 = 1;
pub const SELF_REPORTED: u8 = 2;
pub const REFUSED: u8 = 3;
pub const NO_EVIDENCE: u8 = 4;

/// What an admitted loader carries; bytes 8..80 stay zero when nothing was admitted.
#[derive(Clone, Copy)]
pub struct Enrolled {
    pub measurement: [u8; 32],
    pub root: [u8; 32],
    pub epoch: u64,
}

/// The kernel's verdict, in the record's own terms.
#[derive(Clone, Copy)]
pub enum Answer {
    NotYet,
    Measured(Enrolled),
    SelfReported(Enrolled),
    Refused(u32),
    NoEvidence,
}

pub fn encode(answer: &Answer) -> [u8; RECORD_LEN] {
    let mut r = [0u8; RECORD_LEN];
    r[0] = RECORD_VERSION;
    let (state, enrolled) = match *answer {
        Answer::NotYet => (NOT_YET, None),
        Answer::Measured(e) => (MEASURED, Some(e)),
        Answer::SelfReported(e) => (SELF_REPORTED, Some(e)),
        Answer::Refused(code) => {
            r[4..8].copy_from_slice(&code.to_le_bytes());
            (REFUSED, None)
        }
        Answer::NoEvidence => (NO_EVIDENCE, None),
    };
    r[1] = state;
    if let Some(e) = enrolled {
        r[8..16].copy_from_slice(&e.epoch.to_le_bytes());
        r[16..48].copy_from_slice(&e.measurement);
        r[48..80].copy_from_slice(&e.root);
    }
    r
}
