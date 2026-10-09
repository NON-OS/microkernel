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

//! The two shapes a special key's sequence takes.

use alloc::vec::Vec;
use core::fmt::Write;

use super::keys::Mods;
use crate::out::Out;

/// A key with a final letter: `ESC [ A`, `ESC O A` in application mode,
/// `ESC [ 1 ; m A` with modifiers.
pub(super) fn letter(out: &mut Vec<u8>, fin: char, m: Mods, ss3: bool) {
    if m.any() {
        let _ = write!(Out(out), "\x1b[1;{}{fin}", m.param());
    } else if ss3 {
        let _ = write!(Out(out), "\x1bO{fin}");
    } else {
        let _ = write!(Out(out), "\x1b[{fin}");
    }
}

/// A key numbered with a tilde: `ESC [ n ~`, `ESC [ n ; m ~`.
pub(super) fn tilde(out: &mut Vec<u8>, n: u8, m: Mods) {
    if m.any() {
        let _ = write!(Out(out), "\x1b[{n};{}~", m.param());
    } else {
        let _ = write!(Out(out), "\x1b[{n}~");
    }
}
