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

//! A path the record names is trusted only to stay inside the release: it is
//! relative and every component is an ordinary name. No `..`, no root, no
//! `.`, no empty component, no backslash or control character, so it means the same thing
//! on any host and cannot climb out of the directory being checked.

const MAX_LEN: usize = 4096;

pub fn safe(rel: &str) -> bool {
    !rel.is_empty()
        && rel.len() <= MAX_LEN
        && !rel.chars().any(|c| c == '\\' || c.is_control())
        && rel.split('/').all(|c| !c.is_empty() && c != "." && c != "..")
}

/// A name the record gives a sealed artifact (`esp`, `usb`, `kernel.approval`):
/// short, and only letters, digits, `.`, `_` and `-`, so a reason that quotes
/// it cannot carry anything a terminal would act on.
pub fn safe_name(name: &str) -> bool {
    (1..=64).contains(&name.len())
        && name.bytes().all(|b| b.is_ascii_alphanumeric() || b".-_".contains(&b))
}
