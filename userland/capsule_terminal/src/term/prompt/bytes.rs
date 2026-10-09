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

/// The mark between the prompt and the command, zsh's `%` for an ordinary user.
///
/// The same mark `draw_prompt` puts in front of the line being typed, so what
/// a command looked like while it was entered is what it looks like once it is
/// history: one `user@host path % cmd` line opening the block.
pub const PROMPT_BYTES: &[u8] = b"% ";
