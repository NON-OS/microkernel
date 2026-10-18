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

//! The order the market is asked about listings in, apart from the asking
//! so the proofs can hold it.

use super::listing::{Known, Listing};
use super::verdict::Verdict;

/// How often the gates are asked again while the bond is read, and how far apart.
pub const RECHECKS: u8 = 6;
pub const RECHECK_MS: u64 = 5_000;

/// Which listing to ask about next, and whether it is the selection: the
/// selection first, until its release and gates are known or its recheck is
/// due at `now_ms`, then the first listing whose description is unasked.
/// None when every answer is held.
pub fn next(listings: &[Listing], selected: Option<usize>, now_ms: u64) -> Option<(usize, bool)> {
    let due = |l: &Listing| !l.known.judged || l.known.recheck_at.is_some_and(|t| now_ms >= t);
    if let Some(at) = selected.filter(|&at| listings.get(at).is_some_and(due)) {
        return Some((at, true));
    }
    listings.iter().position(|l| !l.known.described).map(|at| (at, false))
}

/// Ask the gates again in RECHECK_MS while only the bond is unread, at most
/// RECHECKS times; never once the gates are settled either way.
pub fn recheck(k: &mut Known, now: u64) {
    let checking = k.readiness.is_some_and(|r| Verdict::of(&r) == Verdict::Checking);
    k.recheck_at = (checking && k.rechecks < RECHECKS).then(|| now + RECHECK_MS);
    k.rechecks += u8::from(k.recheck_at.is_some());
}
