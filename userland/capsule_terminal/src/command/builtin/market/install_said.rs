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

//! What `market install` says of the system's answer to its install call.

use alloc::format;
use alloc::string::String;

const EBUSY: i64 = -16;
const EINVAL: i64 = -22;

pub(super) fn said(name: &str, rc: i64) -> String {
    match rc {
        0 => format!(
            "market: {name} queued; `market info {name}` shows how it goes, as does the Marketplace"
        ),
        EBUSY => format!("market: {name} is queued already, or the queue is full (EBUSY)"),
        EINVAL => format!("market: the system refused {name} as a listing id (EINVAL)"),
        e => format!("market: the system refused the install (errno {})", -e),
    }
}
