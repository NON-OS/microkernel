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

//! A new wallet is never written over a vault not read yet, and over the
//! account open now or a vault this boot cannot open only when asked twice.

use crate::replace_rule::{
    gave_up, rule, Replace, PATIENCE_MS, REPLACE_OPEN, REPLACE_SEALED, UNREAD, UNREADABLE,
};

#[test]
fn an_unread_vault_is_never_written_over() {
    for open in [false, true] {
        for present in [false, true] {
            assert_eq!(rule(false, false, open, present), Replace::Refuse(UNREAD));
        }
    }
}

#[test]
fn an_open_account_or_a_sealed_vault_is_asked_about() {
    assert_eq!(rule(true, false, true, false), Replace::Ask(REPLACE_OPEN));
    assert_eq!(rule(true, false, true, true), Replace::Ask(REPLACE_OPEN));
    assert_eq!(rule(true, false, false, true), Replace::Ask(REPLACE_SEALED));
}

#[test]
fn an_empty_machine_takes_a_new_wallet() {
    assert_eq!(rule(true, false, false, false), Replace::Free);
}

#[test]
fn a_store_silent_past_the_patience_is_asked_twice_never_refused_forever() {
    assert_eq!(rule(false, true, false, false), Replace::Ask(UNREADABLE));
    assert_eq!(rule(false, true, false, true), Replace::Ask(UNREADABLE));
    assert_eq!(rule(false, true, true, false), Replace::Ask(REPLACE_OPEN));
    for open in [false, true] {
        for present in [false, true] {
            assert_ne!(rule(false, true, open, present), Replace::Free, "never at the first press");
        }
    }
}

#[test]
fn the_patience_runs_from_the_first_silence() {
    assert!(!gave_up(None, i64::MAX), "a store never asked has not been waited on");
    assert!(!gave_up(Some(1_000), 1_000 + PATIENCE_MS - 1));
    assert!(gave_up(Some(1_000), 1_000 + PATIENCE_MS));
    assert!(!gave_up(Some(5_000), 1_000), "a clock that went back waits again");
}
