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

//! Safe Rust surface over the QuickJS runtime. A single owned runtime and
//! context per Engine; values never cross the boundary, only strings.

mod eval;
mod dialog;
mod events;
mod ffi;
mod lifecycle;
mod limits;
mod ready;
mod ui_events;

pub use dialog::{Asked, Dialog};
pub use lifecycle::Engine;
pub use limits::{Limits, Stop};
pub use ui_events::{Key, Press, MOD_ALT, MOD_CTRL, MOD_META, MOD_SHIFT};
