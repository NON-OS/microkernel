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


//! An end settles its kept frames before it is written (L80): finish.rs
//! `put` pinned whole (whole), so the settle cannot move behind the record's
//! write, as text or as a closure called late, nor be keyed or judged
//! otherwise; a lost end leaves its frames for its next end.

use super::code::flat;
use super::whole::whole;

const FINISH: &str = include_str!("../../../../../src/hardware/broker/claim/wipe/finish.rs");

const PUT: [&str; 14] = [
    "pub(super) fn put(w: Wipe, end: Option<End>, now: u64) {",
    "if end == Some(End::Unreset) { super::check::unmark(&ConfigSpace::new(w.address), &w); }",
    "let (freed, kept) = match end { None | Some(End::Lost) => (0, 0),",
    "Some(e) => crate::hardware::broker::dma::settle_kept( w.device_id, w.epoch,",
    "frees(e, w.bme_off, w.caps.binds), ), };",
    "{ let mut all = crate::smp::lock_responsive(&WIPES);",
    "let key = |r: &Wipe| (r.device_id, r.epoch) == (w.device_id, w.epoch);",
    "let Some(at) = all.iter().position(key) else { return };",
    "match end.map(|e| ended(e, w.caps.display, now)) {",
    "None => all[at] = Wipe { busy: false, ..w },",
    "Some(None) => drop(all.remove(at)),",
    "Some(Some(phase)) => all[at] = Wipe { phase, busy: false, ..w }, }",
    "if end.is_some() { PENDING.fetch_sub(1, Ordering::AcqRel); } }",
    "let Some(end) = end else { return }; say(&w, end, freed, kept); }",
];

#[test]
fn an_end_settles_its_frames_before_it_is_written() {
    whole(&flat(FINISH), "put", &PUT);
}
