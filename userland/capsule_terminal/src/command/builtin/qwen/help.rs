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

//! What `help qwen` says under its usage line.

/// The tiers, the models behind them, and the keys of a chat.
pub const HELP: &[&[u8]] = &[
    b"  tiers: small 0.5B (the default), medium 1.5B, large 3B, xlarge 7B",
    b"  all Qwen2.5 Instruct Q4_K_M, run offline, each model verified by SHA-256",
    b"  the rest of the line is the first question; it is not kept in history",
    b"  the answer streams onto the screen as it is written",
    b"  Enter sends a line; Ctrl-D on an empty line ends the chat, Ctrl-C kills it",
    b"  in the chat, /reset starts over and /exit or /bye ends it",
];
