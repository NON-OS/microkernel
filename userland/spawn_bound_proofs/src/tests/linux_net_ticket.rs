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


//! `tool.linux.net` on a ticket: the terminal role's run of a program whose
//! policy grants NET_ANON asked for it, and only its own Terminal, with the
//! argv the kernel set, starts it, once.

use crate::term_ticket_rule::{admit, end_of, Ticket};

fn words(w: &[&str]) -> Vec<String> {
    w.iter().map(|s| s.to_string()).collect()
}

fn ticket() -> Option<Ticket> {
    Some(Ticket { terminal: 40, argv: words(&["linux", "profanity", "-a", "me"]) })
}

#[test]
fn the_ticket_s_terminal_starts_the_ticket_s_argv_once() {
    let mut slot = ticket();
    assert!(admit(&mut slot, 40, &words(&["linux", "profanity", "-a", "me"])));
    assert_eq!(slot, None, "spent");
    assert!(!admit(&mut slot, 40, &words(&["linux", "profanity", "-a", "me"])), "only once");
}

#[test]
fn another_caller_or_another_argv_is_refused_and_spends_the_ticket() {
    for (caller, argv) in [
        (41, words(&["linux", "profanity", "-a", "me"])),
        (40, words(&["linux", "profanity", "-a", "you"])),
        (40, words(&["linux", "profanity", "-a"])),
        (40, words(&["linux", "profanity", "-a", "me", ""])),
        (40, words(&["linux", "sh"])),
        (0, words(&[])),
    ] {
        let mut slot = ticket();
        assert!(!admit(&mut slot, caller, &argv), "{caller} {argv:?}");
        assert_eq!(slot, None, "a wrong ask spends it: {caller} {argv:?}");
    }
}

#[test]
fn no_ticket_admits_nothing_and_a_terminal_s_exit_voids_only_its_own() {
    let mut none = None;
    assert!(!admit(&mut none, 40, &words(&["linux", "profanity", "-a", "me"])));
    let mut slot = ticket();
    end_of(&mut slot, 41);
    assert_eq!(slot, ticket(), "another process's exit leaves it");
    end_of(&mut slot, 40);
    assert_eq!(slot, None);
}
