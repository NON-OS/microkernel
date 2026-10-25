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

//! The self-test's one way to make the personality fail on purpose, so a
//! booted selftest image shows what the Terminal says when capsule_linux
//! panics inside a box: no report, and its `[BOX-MISSING]` line. Compiled in
//! only with the `selftest-panic` feature, which only the selftest
//! development image turns on (tools/nonos-dev-image), and which
//! scripts/check_copy_only_features.py keeps out of every committed build.

/// The word, after `linux --box <box>`, that panics in a selftest build.
#[cfg(feature = "selftest-panic")]
pub const WORD: &[u8] = b"--selftest-panic";

#[cfg(feature = "selftest-panic")]
pub fn maybe(word: Option<&[u8]>) {
    if word == Some(WORD) {
        panic!("selftest: capsule_linux panics on purpose, inside a box");
    }
}

/// Without the feature the word is only a program name nothing holds.
#[cfg(not(feature = "selftest-panic"))]
pub fn maybe(_word: Option<&[u8]>) {}
