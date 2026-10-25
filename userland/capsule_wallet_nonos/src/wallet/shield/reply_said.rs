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

//! What a follow's state says, and how a landing is told.

use alloc::string::String;

use super::SETTLING;

/* What a landing says, with its transaction when the service named one. */
pub fn landed_news(what: &str, tx: &str) -> String {
    if tx.is_empty() {
        alloc::format!("{what} landed.")
    } else {
        alloc::format!("{what} landed ({tx}).")
    }
}

/* Whether a follow's state is the spend's end: in a block, its own or
 * another's. */
pub fn landed(said: &str) -> bool {
    matches!(said, "landed" | "spent elsewhere")
}

/* What a follow's state says, for a spend its owner was or was not settling. */
pub fn said(said: &str, settling: bool) -> &'static str {
    match said {
        "landed" => "landed",
        "spent elsewhere" if settling => "landed from your account",
        "spent elsewhere" => "landed by another route",
        _ if settling => SETTLING,
        "republished" => "handed to a lander again",
        "unpublished" => "kept, no lander took it",
        _ => "waiting for a lander",
    }
}
