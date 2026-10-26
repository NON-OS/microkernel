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

//! Where the port's cursor is: at the start of a line or partway along one.
//! A capsule's MkDebug may send its text and its newline in separate calls,
//! and a whole kernel line written between them would land after the text
//! ("[BLK] step claim[USER-FIRST] pid=..."). A whole line starts by ending
//! such a stranger's partial line, so it always begins at column 0 and the
//! readers that grep these lines find them.

use core::sync::atomic::{AtomicBool, Ordering};

static AT_START: AtomicBool = AtomicBool::new(true);

/// Every byte the port takes passes here, under its lock.
pub fn note(ch: u8) {
    AT_START.store(ch == b'\n', Ordering::Relaxed);
}

/// What a whole line writes before itself: nothing at a line's start, else
/// the newline the partial line before it never sent.
pub fn opening() -> &'static [u8] {
    if AT_START.load(Ordering::Relaxed) {
        b""
    } else {
        b"\r\n"
    }
}
