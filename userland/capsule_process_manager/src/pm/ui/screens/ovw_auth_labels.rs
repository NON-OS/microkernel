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

//! What the four sensitive classes are called on this card.

use super::super::risk_strip::CLASSES;

// Spelt out rather than abbreviated. The authority matrix has room for a legend
// pairing short names with long ones; a card this size does not, so it uses
// words that need no key, in the order the risk strip draws its slots.
pub(super) const LABELS: [&[u8]; 4] = [b"admin", b"raw hw", b"spawn", b"debug"];

// The swatch is the strip's slot, minus its gap, so the eye that learned the
// four positions in the table reads this card without relearning them.
const KEY_H: u32 = 12;

// This card packs its own frame rather than going through `card::paint`, and the
// reason is arithmetic: a caption line plus four rows of text at the toolkit's
// readable floor needs more height than the shared 13px padding leaves in a
// 136px card, and the rows would have overlapped by a few pixels each. The pad
// is tightened here alone, so the card still lines up with its three neighbours
// on the outside while fitting four rows on the inside.

// A label per class, checked at compile time rather than trusted: adding a fifth
// sensitive class to the strip must not silently index past the names here.
const _: () = assert!(CLASSES.len() == LABELS.len());
