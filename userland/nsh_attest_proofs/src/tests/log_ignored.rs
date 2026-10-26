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

//! What the replay leaves alone: events of different PCRs may interleave,
//! and `EV_NO_ACTION` events and PCR 8, which the loader extends outside the
//! log, are not replayed.

use super::passes;
use crate::build::tcg::{EV_APP, EV_NO_ACTION};
use crate::build::tcg_event::{digest, event};
use crate::build::tcg_log::{loader, Log};
use crate::build::Spec;

fn boot() -> [u8; 32] {
    digest(b"boot manager")
}

#[test]
fn events_of_different_pcrs_may_interleave() {
    let mut l = Log::empty();
    l.ext(7, 1, digest(b"a")).ext(4, EV_APP, boot()).ext(7, 1, digest(b"b"));
    l.ext(0, 1, digest(b"c")).ext(4, EV_APP, loader());
    let mut m = Log::empty();
    m.ext(0, 1, digest(b"c")).ext(7, 1, digest(b"a")).ext(4, EV_APP, boot());
    m.ext(4, EV_APP, loader()).ext(7, 1, digest(b"b"));
    assert_eq!(l.pcrs, m.pcrs);
    passes(Spec { log: Log { bytes: m.bytes, pcrs: l.pcrs }.with_capsules(), ..Spec::default() });
}

#[test]
fn no_action_events_and_the_loader_s_pcr_in_the_log_are_not_replayed() {
    let mut s = Spec::default();
    let zeros = vec![(0x0004, vec![0; 20]), (0x000B, vec![0; 32])];
    s.log.bytes.extend(event(4, EV_NO_ACTION, &zeros, b"a note"));
    s.log.ext(8, 0x0D, digest(b"loader's own measurement"));
    passes(s);
}
