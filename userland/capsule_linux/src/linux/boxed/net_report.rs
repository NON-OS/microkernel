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

//! What a box naming destinations adds to its report: the destinations the
//! run connected to, on the report and on one `[BOX-NET]` line before its
//! `[BOX]` line. A box with no destination adds neither, so its report and
//! log are as they were.

use alloc::string::String;
use alloc::vec::Vec;

use super::allowed::dests;
use super::preset::BoxPolicy;
use super::report::VERSION;

/// The report's `connected` line.
pub fn connected(b: &BoxPolicy, reached: &[Vec<u8>]) -> Option<String> {
    (!b.dests.is_empty()).then(|| alloc::format!("connected  {}", joined(reached, " ")))
}

/// The serial log's `[BOX-NET]` line: every destination the box allowed,
/// and every one the run connected to, `none` for none.
pub fn net_line(b: &BoxPolicy, reached: &[Vec<u8>]) -> Option<String> {
    let allowed: Vec<Vec<u8>> = dests(&b.dests).into_iter().map(String::into_bytes).collect();
    (!b.dests.is_empty()).then(|| {
        alloc::format!(
            "[BOX-NET] v={VERSION} allowed={} connected={}\n",
            joined(&allowed, ","),
            joined(reached, ",")
        )
    })
}

fn joined(items: &[Vec<u8>], sep: &str) -> String {
    match items.is_empty() {
        true => "none".into(),
        false => items.iter().map(|d| String::from_utf8_lossy(d)).collect::<Vec<_>>().join(sep),
    }
}
