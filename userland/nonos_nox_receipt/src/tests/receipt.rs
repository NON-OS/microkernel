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

/// A receipt carrying one log with the given parts.
pub fn receipt(status: &str, addr: &str, topics: &[String], data: &str) -> String {
    let mut t = String::new();
    for (i, top) in topics.iter().enumerate() {
        if i > 0 {
            t.push(',');
        }
        t.push('"');
        t.push_str(top);
        t.push('"');
    }
    format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{{\"status\":\"{status}\",\"logs\":[{{\"address\":\"{addr}\",\"topics\":[{t}],\"data\":\"{data}\"}}]}}}}"
    )
}
