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

//! The services policies.txt names, by the word it uses for each.

use crate::mask::{
    EXEC, FILES_HOME, FILES_USER, JIT, MODELS_RO, NET_ANON, NET_LISTEN_LO, TTY, WAYLAND,
};

const NAMES: [(&str, u64); 9] = [
    ("WAYLAND", WAYLAND),
    ("NET_ANON", NET_ANON),
    ("NET_LISTEN_LO", NET_LISTEN_LO),
    ("FILES_HOME", FILES_HOME),
    ("FILES_USER", FILES_USER),
    ("MODELS_RO", MODELS_RO),
    ("EXEC", EXEC),
    ("JIT", JIT),
    ("TTY", TTY),
];

/// The mask bit `word` names, if it names one.
pub fn bit_named(word: &str) -> Option<u64> {
    NAMES.iter().find(|(n, _)| *n == word).map(|(_, b)| *b)
}
