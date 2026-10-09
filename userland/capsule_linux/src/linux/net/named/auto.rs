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

//! The name bind chooses when a Unix socket is given its family alone.

use alloc::vec::Vec;

use crate::linux::net::sock::{self, UName};

/// Linux's autobind: a NUL and five hex digits, the first unused.
pub fn fresh() -> Option<UName> {
    (0u32..0x10_0000).find_map(|n| {
        let mut key: Vec<u8> = alloc::vec![0];
        key.extend_from_slice(alloc::format!("{n:05x}").as_bytes());
        let name = UName { key: key.clone(), shown: key };
        let used = sock::with(|t| t.iter().any(|(_, s)| s.uname.as_ref() == Some(&name)));
        (!used).then_some(name)
    })
}
