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

//! What the marketplace window says when something does not work: each
//! failure names what happened and what to do, in lines the panes hold.

use nonos_market_proto::reason;
use nonos_market_proto::status::{E_INVAL, E_KEYREJECTED, E_MSGSIZE, E_NODATA};

use crate::store::failure::Failure;
use crate::store::refused::refused;

#[test]
fn each_refusal_of_the_system_has_its_own_words() {
    let inval = refused(-22);
    let perm = refused(-1);
    let other = refused(-5);
    assert_ne!(inval, perm);
    assert_ne!(perm, other);
    assert_eq!(refused(-13), perm, "EACCES reads as EPERM: the window may not ask");
    assert!(core::str::from_utf8(perm).unwrap().contains("AppInstall"));
    assert!(core::str::from_utf8(other).unwrap().contains("serial log"), "where to look");
}

#[test]
fn every_catalogue_failure_says_what_to_do_in_lines_the_pane_holds() {
    let all = [
        Failure::NoReply,
        Failure::Mismatch,
        Failure::Malformed,
        Failure::Status(E_NODATA),
        Failure::Status(E_INVAL),
        Failure::Status(E_MSGSIZE),
        Failure::Status(E_KEYREJECTED),
        Failure::Status(-999),
    ];
    for f in all {
        let said = String::from_utf8(f.catalogue_trouble()).unwrap();
        assert!(said.lines().count() >= 2, "{f:?}: what happened, then what to do: {said}");
        for line in said.lines() {
            assert!(line.len() <= 60, "{f:?}: {line:?} is {} long", line.len());
        }
    }
}

#[test]
fn an_unknown_reason_is_still_a_sentence_with_a_next_step() {
    let said = reason(u8::MAX);
    assert!(said.retry);
    assert!(said.line.contains("retry") && said.line.contains("serial log"), "{}", said.line);
}
