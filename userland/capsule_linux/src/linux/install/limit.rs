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

//! How many packages one install may bring in.
//!
//! A bound, because a closure that names half a distribution is an index
//! gone wrong, not a choice. Large, because a tool group does pull hundreds:
//! a BlackArch group is several hundred packages. An image may set its own
//! in /etc/nonos-install-max, and never above the ceiling.

use crate::linux::file::{key, store_read};

const DEFAULT: usize = 1024;
const CEILING: usize = 4096;
const FILE: &[u8] = b"/etc/nonos-install-max";

pub(super) fn max_packages() -> usize {
    let Ok(raw) = store_read(&key(FILE), 16) else {
        return DEFAULT;
    };
    let text = raw.split(|b| b.is_ascii_whitespace()).next().unwrap_or(&[]);
    let parsed = text.iter().try_fold(0usize, |v, b| match b {
        b'0'..=b'9' => v.checked_mul(10)?.checked_add((b - b'0') as usize),
        _ => None,
    });
    match parsed {
        Some(n) if n > 0 => n.min(CEILING),
        _ => DEFAULT,
    }
}
