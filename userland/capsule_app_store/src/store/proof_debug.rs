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

//! The store's `[STORE-PROVE]` serial lines (I-B9a, section 5), pid-free,
//! heard only with the optional Debug cap. None starts `[PROVE] syscall
//! buckets`, so the boot verdict counts the prover's one set alone.

use nonos_libc::mk_debug;

use super::proof_fail::decimal;
use super::proof_seq::{Phase, DEVICE, ISOLATION, READING};
use super::proof_state::Run;

/// `[STORE-PROVE] ` and the parts, as one line.
pub fn say(parts: &[&[u8]]) {
    let line = [&[&b"[STORE-PROVE] "[..]], parts].concat().concat();
    let _ = mk_debug(line.as_ptr(), line.len());
}

/// The stage's word, for a stage the job works in.
pub fn stage(stage: u8) -> Option<&'static [u8]> {
    match stage {
        READING => Some(b"reading"),
        DEVICE => Some(b"device"),
        ISOLATION => Some(b"isolation"),
        _ => None,
    }
}

/// `held: linux.<app>, <n> bytes, not sent`: n is at most the 1 MiB a
/// container can be.
pub fn held(listing: &[u8], n: usize) {
    let n = decimal(i32::try_from(n).unwrap_or(i32::MAX));
    say(&[b"held: ", listing, b", ", &n, b" bytes, not sent"]);
}

/// The serial line for where the run went from `was`, if any.
pub fn told(run: &Run, was: Phase) {
    match (was, run.phase) {
        (Phase::Asked, Phase::Up) => say(&[b"prover up"]),
        (Phase::Proving { stage: s0, .. }, Phase::Proving { stage: s, .. }) if s == s0 => {}
        (_, Phase::Proving { stage: s, .. }) => stage(s).map_or((), |w| say(&[b"stage ", w])),
        (Phase::Failed(..), _) => {}
        (_, Phase::Failed(f, _)) if !run.cancelled => {
            say(&[b"failed: ", &run.listing, b", ", &f.code()])
        }
        _ => {}
    }
}
