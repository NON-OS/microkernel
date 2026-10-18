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

//! What a hostile sink may lie about: its segment pointer, and one try's
//! read.

/// How the sink treats the segment pointer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eddc {
    Honest,
    /// No E-DDC: the pointer's address is not acknowledged.
    Nak,
    /// Acknowledged, then forgotten: every read is from segment 0.
    Ignore,
    /// Acknowledged, then read from the segment this far past it.
    Off(u8),
}

/// One try's lie, in its read.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Fault {
    #[default]
    None,
    /// NAK when this many words are taken.
    Nak(usize),
    /// Never ready again once this many words are taken.
    Stall(usize),
    /// This byte is lost: the rest arrive one early.
    Drop(usize),
    /// This word comes twice: the rest arrive four late.
    Again(usize),
    /// The read starts at this index, not the one asked.
    Index(u8),
    /// Each word holds the clock this many microseconds.
    Slow(u64),
    /// The wait phase never shows after the last word.
    NoPhase,
    /// Words still offered after the last asked.
    Over,
}
