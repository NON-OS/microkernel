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
//! Taking a file out with what the machine wrote beside it (minted.rs), in
//! the order that keeps a program's box while it can still start.

use super::minted::{beside, POLICY, POLICY_TRAILER, TRAILER};

/// Take `at` out with what was written beside it, through `unlink` and
/// `found`, the store's look. A path is gone only when the store says it is
/// not there: a store that fails both says nothing, and so keeps the box.
/// The policy goes only once the program cannot start, its file or its
/// trailer gone; until then it stays and keeps the program in its box. True
/// when the file is gone.
pub fn take_out<E: PartialEq>(
    at: &[u8],
    unlink: &mut impl FnMut(&[u8]),
    found: &mut impl FnMut(&[u8]) -> Result<bool, E>,
) -> bool {
    let mut gone = |p: &[u8]| {
        unlink(p);
        found(p) == Ok(false)
    };
    let file = gone(at);
    let trailer = gone(&beside(at, TRAILER));
    if file || trailer {
        gone(&beside(at, POLICY_TRAILER));
        gone(&beside(at, POLICY));
    }
    file
}
