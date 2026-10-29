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

//! The frame that carries a command: `{"corrId":"<id>","cmd":"<command>"}`.

use alloc::format;
use alloc::string::String;

use super::json::push_quoted;

/// The WebSocket text frame for command `text` under correlation id `corr`.
pub fn frame(corr: u32, text: &str) -> String {
    let mut out = format!("{{\"corrId\":\"{corr}\",\"cmd\":");
    push_quoted(&mut out, text);
    out.push('}');
    out
}
