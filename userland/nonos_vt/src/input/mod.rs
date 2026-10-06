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

//! What a program reads when a person types, points or pastes, encoded as
//! xterm encodes it under the modes the program set.

mod keys;
mod keys_encode;
mod keys_seq;
mod mouse;
mod mouse_encode;
mod paste;

pub use keys::{Key, Mods};
pub use keys_encode::encode_key;
pub use mouse::{Button, MouseEvent, MouseKind};
pub use mouse_encode::{encode_focus, encode_mouse};
pub use paste::encode_paste;
