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

/*
 * Which number the service keeps each earlier spend under, as the wallet
 * learns it. Pure, so wallet_proofs holds the rule.
 */

use alloc::vec::Vec;

/* The numbers `reported` names that the wallet does not know, split in
 * two: those older than this window's own spends, which the wallet follows
 * as payments it did not see proved, and the newest `unnamed`, which are
 * this window's unnamed spends, in the same order. */
pub fn split<'a>(
    known: &[&str],
    unnamed: usize,
    reported: &[&'a str],
) -> (Vec<&'a str>, Vec<&'a str>) {
    let fresh: Vec<&'a str> = reported.iter().copied().filter(|r| !known.contains(r)).collect();
    let at = fresh.len().saturating_sub(unnamed);
    let (older, newer) = fresh.split_at(at);
    (older.to_vec(), newer.to_vec())
}
