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

pub(crate) const WORDS: usize = 64;
pub(super) const REQUEST: u32 = 0;
pub(super) const ANSWERED: u32 = 0x8000_0000;
pub(super) const TAG_ANSWERED: u32 = 1 << 31;

/// One tag to ask: its request values, and room for its answer in words.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Tag<'a> {
    pub(crate) id: u32,
    pub(crate) values: &'a [u32],
    pub(crate) room: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MessageError {
    TooLarge,
    /// The buffer came back without the firmware's success code.
    NotAnswered,
    /// The tag came back without its answered bit, or longer than its room.
    TagNotAnswered,
}

impl Tag<'_> {
    /// Words the tag's value buffer takes: enough for request and answer.
    pub(super) fn room_words(&self) -> usize {
        self.room.max(self.values.len())
    }
}
