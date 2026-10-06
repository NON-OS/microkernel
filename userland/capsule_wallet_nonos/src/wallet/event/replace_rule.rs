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

//! The rule for writing a new wallet over what this machine holds, pure so
//! wallet_proofs holds it.

/// What a make, import or restore may do now.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Replace {
    /// Nothing is there to lose: go on.
    Free,
    /// Something is replaced: ask once, and go on at the second press.
    Ask(&'static str),
    /// The vault was not read to an answer yet: refuse.
    Refuse(&'static str),
}

pub const UNREAD: &str = "A wallet may be stored on this machine and it has not been read yet. \
     The wallet reads it by itself in a moment; nothing was changed.";
pub const UNREADABLE: &str = "This machine's stored wallet could not be read, so one may \
     still be stored here. A new wallet works now, but saving it here writes over whatever is \
     stored, and that wallet's recovery words are then the only way back to it. Press again \
     to make the new one.";
pub const REPLACE_OPEN: &str = "This replaces the account open now on this machine. Its key \
     stays only in your backup. Press again to replace it.";
pub const REPLACE_SEALED: &str = "A wallet is stored here that this boot cannot open. This \
     writes over it, and its recovery words are then the only way back to it. Press again \
     to replace it.";

/// How long a store that never answers is waited on before the machine is
/// taken as one whose stored wallet cannot be read. A store staging packages
/// at boot answers well inside it; one that is broken never does, and a
/// wallet must not be refused forever on its account.
pub const PATIENCE_MS: i64 = 20_000;

/// Whether the vault's store has been silent for the whole patience, from
/// `silent_since`, the uptime of its first silent answer.
pub fn gave_up(silent_since: Option<i64>, now: i64) -> bool {
    silent_since.is_some_and(|since| now.saturating_sub(since) >= PATIENCE_MS)
}

/// `read` is the vault read to an answer, `waited` the store silent for the
/// whole patience, `open` an account open now, and `present` a vault on the
/// disk this boot could not open or name. Nothing that may be stored is
/// ever written over at the first press.
pub fn rule(read: bool, waited: bool, open: bool, present: bool) -> Replace {
    if !read && !waited {
        return Replace::Refuse(UNREAD);
    }
    if open {
        return Replace::Ask(REPLACE_OPEN);
    }
    if !read {
        return Replace::Ask(UNREADABLE);
    }
    if present {
        return Replace::Ask(REPLACE_SEALED);
    }
    Replace::Free
}
