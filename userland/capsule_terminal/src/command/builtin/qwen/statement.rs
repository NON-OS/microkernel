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

//! `qwen` reached through the shell's parser: after `;`, `&&` or `||`, from
//! an alias, or in a pipe or a redirect. The parser has already split and
//! expanded whatever followed it, so only the command and a tier are taken
//! this way; a question is refused rather than sent changed.

use super::tiers::TIERS;
use crate::jobs::JobWork;
use crate::term::state::State;

/// `qwen` or `qwen <tier>` as a parsed statement. `None`, with the reason on
/// screen, for anything else or when the kernel refuses.
pub fn from_args(state: &mut State, args: &[&[u8]]) -> Option<JobWork> {
    let tier = match args {
        [_] => Some(TIERS[0]),
        [_, word] => TIERS.iter().copied().find(|t| t == word),
        _ => None,
    };
    match tier {
        Some(tier) => super::start::spawn(state, tier, b""),
        None => {
            misplaced(state);
            None
        }
    }
}

/// Refuse a `qwen` the parser has changed, and say where it belongs.
pub fn misplaced(state: &mut State) {
    state.scrollback.push_error(
        b"qwen: to ask, start the line with qwen; it cannot run in a pipe or redirect (EINVAL)",
    );
    state.last_status = 1;
}
