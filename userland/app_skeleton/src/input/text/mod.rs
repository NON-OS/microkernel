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

//! Text input for the apps' fields: which keys type which characters, which
//! key pastes, what the clipboard's text becomes in a one-line field, and
//! UTF-8 edits on a fixed buffer. One reading, so every field agrees on what
//! a key does.

mod paste_key;
mod paste_line;
mod typed;
mod utf8;

pub use paste_key::is_paste;
pub use paste_line::{first_line, paste_char, PasteLine};
pub use typed::{text_char, typed_char};
pub use utf8::{last_char_len, push_char};
