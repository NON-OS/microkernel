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

use super::listing::Listing;

/// Which listing to ask about next, and whether it is the selection: the
/// selection first, until its release and gates are known, then the first
/// listing whose description is unasked. None when every answer is held.
pub fn next(listings: &[Listing], selected: Option<usize>) -> Option<(usize, bool)> {
    if let Some(at) = selected.filter(|&at| listings.get(at).is_some_and(|l| !l.known.judged)) {
        return Some((at, true));
    }
    listings.iter().position(|l| !l.known.described).map(|at| (at, false))
}
