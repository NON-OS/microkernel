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

use nonos_vt::Term;

use super::types::Scrollback;
use crate::term::dimensions::{COLS, SCROLLBACK_ROWS, VISIBLE_ROWS};

impl Scrollback {
    /// The size here is only where it starts: the paint resizes it to the
    /// window before anything is drawn.
    pub fn new() -> Self {
        Self {
            capture: None,
            vt: Term::new(COLS, VISIBLE_ROWS, SCROLLBACK_ROWS),
            onlcr: true,
            theme_of: None,
        }
    }
}
