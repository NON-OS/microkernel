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

//! An input event as the router builds one.

use crate::input::{InputEvent, InputKind};

pub fn event(kind: InputKind, x: i32, y: i32) -> InputEvent {
    InputEvent { kind, flags: 0, code: 0, x, y, delta_x: 0, delta_y: 0, timestamp_ns: 0 }
}
