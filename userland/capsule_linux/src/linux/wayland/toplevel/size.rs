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

//! The size a toplevel is offered in its first configure. A window of its
//! own is offered the size a guest's window has always opened at here, kept
//! within its own limits and what the display holds: LibreOffice and others
//! size their main window only from what they are told, and told 0 by 0
//! open at their least size. A dialog (a toplevel with a parent) is told 0
//! by 0 and takes its own size, as file choosers and message boxes expect.
//! Pure, so the host proofs hold it.

/// A guest's window as it has always opened.
pub const OFFER: (u32, u32) = (800, 600);

/// `limits` are the least and most size (0 for none), `bounds` what the
/// display holds (0 for not known).
pub fn offered(parent: bool, limits: ((u32, u32), (u32, u32)), bounds: (u32, u32)) -> (u32, u32) {
    if parent {
        return (0, 0);
    }
    let ((lw, lh), (mw, mh)) = limits;
    (within(OFFER.0, lw, mw, bounds.0), within(OFFER.1, lh, mh, bounds.1))
}

/// `v` no more than `most` or `bound` where they are given, and no less
/// than `least`, which wins: a window that cannot be smaller is not told to.
fn within(v: u32, least: u32, most: u32, bound: u32) -> u32 {
    let v = if most > 0 { v.min(most) } else { v };
    let v = if bound > 0 { v.min(bound) } else { v };
    v.max(least)
}
