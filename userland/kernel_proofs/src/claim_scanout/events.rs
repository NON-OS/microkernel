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

//! What the takeover model's steps did (L79), as bits, so a search can show
//! the cases that matter were reached, not only that nothing broke.

/// A take-back's release settled bootfb's claim.
pub(super) const TOOK_BACK: u16 = 1;
/// A covering claim's admit told to ask again.
pub(super) const ASKED_AGAIN: u16 = 1 << 1;
/// A covering claim refused at its admit because its holder exited.
pub(super) const ORPHANED: u16 = 1 << 2;
/// bootfb's claim refused at its entry, the scanout taken.
pub(super) const REFUSED_ENTER: u16 = 1 << 3;
/// bootfb's claim refused at its admit, the scanout taken meanwhile.
pub(super) const REFUSED_ADMIT: u16 = 1 << 4;
/// bootfb's claim met another of its own still standing or going.
pub(super) const BUSY: u16 = 1 << 5;
/// bootfb gave its claim back itself.
pub(super) const GAVE_BACK: u16 = 1 << 6;
/// bootfb's sibling unmapped its grant.
pub(super) const UNMAPPED: u16 = 1 << 7;
/// bootfb stored to the framebuffer, or to the sink.
pub(super) const STORED_FB: u16 = 1 << 8;
pub(super) const STORED_SINK: u16 = 1 << 9;
/// The kernel drew, or backed out at its re-check.
pub(super) const DREW: u16 = 1 << 10;
pub(super) const BACKED_OUT: u16 = 1 << 11;
/// bootfb's exit settled its claim.
pub(super) const EXITED: u16 = 1 << 12;
/// A repoint moved a page to the sink.
pub(super) const REPOINTED: u16 = 1 << 13;
/// A covering claim made ready.
pub(super) const ADMITTED: u16 = 1 << 14;
