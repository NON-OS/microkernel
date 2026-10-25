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

//! What the enter model's steps saw (L78), as bits gathered over a search,
//! so each test can show the cases it speaks of were reached at all.

/// What the steps saw, for the floors: Resetting said, a claim refused, a
/// claim withdrawn, a Lost record asked again, a wipe's end (Clean, Unreset
/// or Lost), and who stepped it there: A's release or exit, the tick, a
/// claimant's enter, or (ResetInMaking) the claimant waiting in Making.
pub(super) const AGAIN: u16 = 1;
pub(super) const REFUSED: u16 = 2;
pub(super) const WITHDRAWN: u16 = 4;
pub(super) const REARMED: u16 = 8;
pub(super) const CLEAN: u16 = 16;
pub(super) const UNRESET: u16 = 32;
pub(super) const LOST: u16 = 64;
pub(super) const BY_A: u16 = 128;
pub(super) const BY_TICK: u16 = 256;
pub(super) const BY_CLAIM: u16 = 512;
pub(super) const BY_WAIT: u16 = 1024;
/// AlreadyClaimed said, over a claim standing.
pub(super) const TAKEN: u16 = 2048;
