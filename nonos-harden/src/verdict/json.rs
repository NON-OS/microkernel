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

//! The verdict as programs read it: format `nonos-harden-verdict/1`, pretty
//! JSON and a trailing newline. These exact bytes are what an operator signs.

use super::Verdict;

pub fn render(v: &Verdict) -> Result<String, String> {
    let body = serde_json::to_string_pretty(v);
    body.map(|s| s + "\n").map_err(|e| format!("the verdict did not serialize ({e})"))
}
