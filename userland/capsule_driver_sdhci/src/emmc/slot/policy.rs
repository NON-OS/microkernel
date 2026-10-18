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

//! What a removable slot does on a look at its card detect: pure, so the
//! host proofs hold every case.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    /// Nothing changed.
    Keep,
    /// The card left: drop it.
    Drop,
    /// The card left and one is in the slot again: drop it, bring that one up.
    Swap,
    /// A card is in the slot and none is up: bring it up.
    BringUp,
    /// The slot is empty and was.
    Empty,
}

/// `have`: a card is up. `inserted`: card detect reads a card, settled.
/// `removed`: the host latched a removal since the last look. `tried`: a
/// bring-up already failed on the card in the slot, which is not tried
/// again until it is taken out, so a broken card costs one attempt.
pub const fn step(have: bool, inserted: bool, removed: bool, tried: bool) -> Step {
    match (have, inserted, removed, tried) {
        (true, false, _, _) => Step::Drop,
        (false, false, _, _) => Step::Empty,
        (true, true, true, _) => Step::Swap,
        (true, true, false, _) => Step::Keep,
        (false, true, true, _) | (false, true, false, false) => Step::BringUp,
        (false, true, false, true) => Step::Keep,
    }
}
