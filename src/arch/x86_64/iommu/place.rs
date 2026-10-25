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

//! The order of a map's leaf writes and its flush, for VT-d and AMD-Vi alike
//!. Pure over its three steps, and proven on the host
//! (kernel_proofs::iommu_place). A failure after the first leaf takes the
//! written leaves back and flushes before the caller may free a frame; if a
//! leaf cannot be cleared, or that flush fails too, the map is Stranded and
//! its frames must never be reused.

/// Why a map did not complete.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Unplaced<E> {
    /// Nothing written is reachable any more: the frames may be freed.
    Failed(E),
    /// The unit may still translate part of it: the frames stay leaked.
    Stranded,
}

/// Write `pages` leaves, then flush. `write(p)` writes leaf p; `clear(p)`
/// clears a written leaf p, false when it cannot; `flush` makes the unit's
/// view match the tables.
pub fn place<E>(
    pages: u64,
    mut write: impl FnMut(u64) -> Result<(), E>,
    mut clear: impl FnMut(u64) -> bool,
    mut flush: impl FnMut() -> Result<(), E>,
) -> Result<(), Unplaced<E>> {
    for page in 0..pages {
        if let Err(e) = write(page) {
            return Err(undo(page, e, &mut clear, &mut flush));
        }
    }
    match flush() {
        Ok(()) => Ok(()),
        Err(e) => Err(undo(pages, e, &mut clear, &mut flush)),
    }
}

/// The first `written` leaves cleared, then the same flush again.
fn undo<E>(
    written: u64,
    e: E,
    clear: &mut impl FnMut(u64) -> bool,
    flush: &mut impl FnMut() -> Result<(), E>,
) -> Unplaced<E> {
    if (0..written).all(clear) && flush().is_ok() {
        Unplaced::Failed(e)
    } else {
        Unplaced::Stranded
    }
}
