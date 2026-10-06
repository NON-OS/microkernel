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
 * What the name step holds between keys.
 */

use super::rules::{Refused, NAME_MAX, SYSTEM_NAME};

pub struct NameState {
    pub line: [u8; NAME_MAX],
    pub len: usize,
    /* The last key kept out and why, until the next key goes in. */
    pub refused: Option<(u8, Refused)>,
}

impl NameState {
    pub const fn new() -> Self {
        Self { line: [0; NAME_MAX], len: 0, refused: None }
    }

    /* What was typed, which is what the policy is told; empty is allowed. */
    pub fn typed(&self) -> &[u8] {
        &self.line[..self.len]
    }

    /* The name the Terminal shows. */
    pub fn shown(&self) -> &[u8] {
        if self.len == 0 {
            SYSTEM_NAME
        } else {
            self.typed()
        }
    }
}
