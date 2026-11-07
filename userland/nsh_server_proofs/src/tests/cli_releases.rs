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

//! The accepted releases as `nsh` reads them, one `release` line each.

use crate::kit::host::built;

pub fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

pub fn releases() -> String {
    built()
        .releases
        .iter()
        .map(|r| {
            let net: Vec<String> = r.network.iter().map(|m| hex(m)).collect();
            let net = if net.is_empty() { "-".into() } else { net.join(",") };
            format!(
                "release {} {} {} {} {} {net}\n",
                r.version,
                hex(&r.pcr8),
                hex(&r.pcr9),
                hex(&r.loader),
                hex(&r.front)
            )
        })
        .collect()
}
