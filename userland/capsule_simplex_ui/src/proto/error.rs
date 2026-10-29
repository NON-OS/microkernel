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

//! An error answer's words: the error's type and, when it has one, the type
//! of the fault under it.

use alloc::format;
use alloc::string::String;

use super::json::Value;

pub fn error(v: &Value) -> String {
    let e = v.get("chatError");
    let kind = e.get("type").str().unwrap_or("error");
    let detail = e.at(&["errorType", "type"]).str().or(e.at(&["storeError", "type"]).str());
    match detail {
        Some(d) => format!("{kind} {d}"),
        None => String::from(kind),
    }
}
