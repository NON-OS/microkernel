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

//! How tall wrapped text would be, drawing nothing.

use alloc::string::String;

use zeroize::Zeroizing;

use super::joined;
use crate::wallet::etna::roles::Role;
use crate::wallet::etna::text::{line, width};

/// The height `text` wrapped to `w` would take, drawing nothing.
pub fn wrapped_height(w: i32, role: Role, text: &str) -> i32 {
    let mut rows = 0;
    let mut row = Zeroizing::new(String::new());
    for word in text.split(' ') {
        let trial = joined(&row, word);
        if !row.is_empty() && width(role, &trial) > w {
            rows += 1;
            row = joined("", word);
        } else {
            row = trial;
        }
    }
    if !row.is_empty() {
        rows += 1;
    }
    rows * line(role)
}
