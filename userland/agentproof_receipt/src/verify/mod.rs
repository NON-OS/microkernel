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

mod capsules;
mod doc;
mod pin;
mod ran;
mod refusal;
mod release;
mod run;
mod verdict;

pub use pin::{pin, FIRST_USE};
pub use ran::Ran;
pub use refusal::Refusal;
pub use release::Release;
pub use verdict::Verdict;

use alloc::string::String;

use agentproof_policy::boxed::preset::Preset;

use crate::napr::read::parse;
use crate::trust::Trust;

pub fn verify(bytes: &[u8], trust: &Trust) -> Result<Verdict, Refusal> {
    let r = parse(bytes).map_err(Refusal::Format)?;
    let d = doc::doc(&r, trust)?;
    let run = run::run(&r, trust)?;
    let ran = ran::ran(r.entries);
    ran::launched(r.launcher, &ran).then_some(()).ok_or(Refusal::Launcher)?;
    Ok(Verdict {
        release: d.release.version.clone(),
        pcr9: *d.doc.pcr(9).ok_or(Refusal::Release)?,
        is_release: release::release(&d.capsules),
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
