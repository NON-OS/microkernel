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
//! A receipt checked: the machine's half first, the doc a trusted kernel
//! signed for this receipt's bytes, then the run's half those bytes carry.

mod doc;
mod pin;
mod ran;
mod refusal;
mod run;

pub use pin::{pin, FIRST_USE};
pub use ran::Ran;
pub use refusal::Refusal;

use alloc::string::String;
use alloc::vec::Vec;

use agentproof_policy::boxed::preset::Preset;

use crate::napr::read::parse;
use crate::trust::Trust;

/// What a verified receipt says. `enforced` is false for a learn run, which
/// held the program to no policy of its own and is never one to rely on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verdict {
    pub release: String,
    pub pcr9: [u8; 32],
    pub box_word: String,
    pub box_hash: [u8; 32],
    pub enforced: bool,
    /// None for the personality's own BusyBox, whose note names no program.
    pub program: Option<[u8; 32]>,
    pub app_id: [u8; 32],
    pub built_in: bool,
    pub supervisor: [u8; 32],
    pub seal_no: u64,
    /// The measurement of the capsule that asked, as nonos.prove read it
    /// from the kernel's registry and one of `ran`; zero if none is named.
    pub launcher: [u8; 32],
    /// Every capsule the machine ran when the doc was made.
    pub ran: Vec<Ran>,
}

pub fn verify(bytes: &[u8], trust: &Trust) -> Result<Verdict, Refusal> {
    let r = parse(bytes).map_err(Refusal::Format)?;
    let (d, release) = doc::doc(&r, trust)?;
    let run = run::run(&r, trust)?;
    let ran = ran::ran(r.entries);
    ran::launched(r.launcher, &ran).then_some(()).ok_or(Refusal::Launcher)?;
    Ok(Verdict {
        release: release.version.clone(),
        pcr9: *d.pcr(9).ok_or(Refusal::Release)?,
        box_word: String::from_utf8_lossy(r.box_word).into_owned(),
        box_hash: run.box_hash,
        enforced: run.policy.preset != Preset::Learn,
        program: run.program,
        app_id: run.app_id,
        built_in: run.built_in,
        supervisor: run.record.sup_meas,
        seal_no: run.record.seal_no,
        launcher: *r.launcher,
        ran,
    })
}
