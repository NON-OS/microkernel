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

//! The terminal size a guest is shown. A guest may set one with TIOCSWINSZ,
//! as `stty rows` does; it holds until the terminal itself changes size,
//! which is newer than what the guest said. Pure, so the host proofs hold
//! it.

/// A size a guest set as (rows, cols), with the terminal's own size then.
pub type Set = Option<((u16, u16), (u16, u16))>;

/// The size shown while the terminal's own is `terminal`.
pub fn shown(set: Set, terminal: (u16, u16)) -> (u16, u16) {
    match set {
        Some((asked, then)) if then == terminal => asked,
        _ => terminal,
    }
}
