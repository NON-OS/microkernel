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

//! What each caller of the model does, in order. H holds two runs, the
//! second split by its cap and the third by a gap; R drains, then reads
//! where H wrote.

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(super) enum Op {
    Enter,
    Hold(u64),
    Flush,
    Give,
    Read(u64),
    Leave,
}

use Op::*;

pub(super) const H: usize = 0;
pub(super) const CAP: u64 = 2;
pub(super) const PROGRAM: [&[Op]; 3] = [
    &[Enter, Hold(10), Hold(11), Hold(12), Hold(20), Flush, Give],
    &[Enter, Hold(30), Flush, Give],
    &[Enter, Flush, Read(10), Read(20), Leave],
];
pub(super) const WRITER: [bool; 3] = [true, true, false];
