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

/*
 * There is no list kind: a list line keeps its "- " or "1. " marker in the
 * text (doc::list), so it is a Paragraph here, and the Markdown export writes
 * it back as the list it was typed as.
 */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlockKind {
    Paragraph,
    Heading(u8),
    PageBreak,
}

impl BlockKind {
    pub fn heading_level(&self) -> Option<u8> {
        match self {
            BlockKind::Heading(n) => Some(*n),
            _ => None,
        }
    }
}
