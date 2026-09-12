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

//! Desktop chrome an application has to work around.
//!
//! One definition. The menubar height was written out separately in the drag
//! code and the maximise code, both as 28, while the shell actually draws 46.
//! So a window dragged to the top slid eighteen pixels under the bar, and a
//! maximised one left a strip of desktop showing above it. Three numbers for
//! one bar is three chances to be wrong about it.

/// Height of the desktop menubar, matching the shell's own `MENUBAR_H_LOGICAL`.
///
/// If the shell's bar changes, this changes with it. There is no way to read it
/// at runtime today, so it is mirrored here and named so the pair can be found.
pub const MENUBAR_H: u32 = 46;
