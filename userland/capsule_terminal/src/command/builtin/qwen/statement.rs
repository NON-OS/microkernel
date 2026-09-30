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
//! expanded whatever followed it, so only the command and a tier, `window`
//! and a tier, `get` and tiers, or `tiers` are taken this way; a question
//! is refused rather than sent changed.

use super::chosen::chosen;
use super::fetch_words::{Fetch, GET, LIST};
use super::tiers::TIERS;
use super::window::{Window, WORD};
use crate::jobs::JobWork;
use crate::term::state::State;

/// `qwen`, `qwen <tier>` or `qwen window [tier]` as a parsed statement.
/// `None`, for a window once it is asked for, and with the reason on
/// screen, for anything else or when the kernel refuses.
pub fn from_args(state: &mut State, args: &[&[u8]]) -> Option<JobWork> {
    let tier = match args {
        [_, word, rest @ ..] if *word == WORD => {
            super::open::ask(state, window(rest));
            return None;
        }
        [_, word, rest @ ..] if *word == GET => {
            return super::fetch::start(state, &Fetch::Get(rest.to_vec()));
        }
        [_, word] if *word == LIST => return super::fetch::start(state, &Fetch::List),
        [_] => Some(chosen()),
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

/// What followed `qwen window` in a statement: nothing, or one tier word.
fn window<'a>(rest: &[&'a [u8]]) -> Window<'a> {
    match rest {
        [] => Window::Open(chosen()),
        [word] => {
            TIERS.iter().copied().find(|t| t == word).map_or(Window::NotATier(word), Window::Open)
        }
        [word, ..] => Window::NotATier(word),
    }
}

/// Refuse a `qwen` the parser has changed, and say where it belongs.
pub fn misplaced(state: &mut State) {
    state.scrollback.push_error(
        b"qwen: to ask, start the line with qwen; it cannot run in a pipe or redirect (EINVAL)",
    );
    state.last_status = 1;
}
