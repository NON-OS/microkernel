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

//! A load's outcome kept for its sender, and told back when it asks: the
//! Terminal stops waiting on a load after 40 ms and asks afterwards, so a
//! refusal must still reach it with its reason, and only it.

use crate::fixtures::{put_artifacts, NAME};
use crate::frame::{by_name_payload, frame, reply};
use crate::handlers::load_outcome::{load_noted, load_outcome};
use crate::protocol::{decode_request, EINVAL, ENOENT, OP_LOAD_BY_NAME, OP_LOAD_OUTCOME};
use crate::server::outcomes::{Outcomes, KEPT};
use crate::syscall;

fn load(kept: &mut Outcomes, sender: u32) -> Vec<u8> {
    let f = frame(OP_LOAD_BY_NAME, &by_name_payload(0x18, &[]));
    load_noted(decode_request(&f).expect("decodes"), sender, kept)
}

fn asked(kept: &Outcomes, sender: u32, payload: &[u8]) -> Vec<u8> {
    let f = frame(OP_LOAD_OUTCOME, payload);
    load_outcome(decode_request(&f).expect("decodes"), sender, kept)
}

#[test]
fn a_refused_load_is_told_to_its_sender_with_its_reason_and_to_no_one_else() {
    let mut kept = Outcomes::new();
    put_artifacts(NAME, Some(2));
    assert_eq!(load(&mut kept, 40), reply(EINVAL, &[]), "a missing manifest is refused");
    assert_eq!(asked(&kept, 40, &by_name_payload(0, &[])), reply(EINVAL, &[]));
    assert_eq!(asked(&kept, 41, &by_name_payload(0, &[])), reply(ENOENT, &[]), "not 41's");
    put_artifacts(NAME, None);
    syscall::reset(55);
    assert_eq!(load(&mut kept, 40), reply(0, &55u32.to_le_bytes()));
    assert_eq!(asked(&kept, 40, &by_name_payload(0, &[])), reply(0, &[]), "the newer load");
    assert_eq!(asked(&kept, 40, &[0; 8]), reply(EINVAL, &[]), "no name: malformed");
}

#[test]
fn the_table_keeps_the_last_loads_and_overwrites_the_oldest() {
    let mut kept = Outcomes::new();
    for i in 0..=KEPT as u32 {
        kept.note(i, b"app", -(i as i32) - 1);
    }
    assert_eq!(kept.of(0, b"app"), None, "the oldest went");
    assert_eq!(kept.of(1, b"app"), Some(-2));
    assert_eq!(kept.of(KEPT as u32, b"app"), Some(-(KEPT as i32) - 1));
    assert_eq!(kept.of(1, b"ap"), None, "a name is matched whole");
    kept.note(1, &[b'a'; 65], -13);
    assert_eq!(kept.of(1, &[b'a'; 65]), None, "no name longer than a store name");
}

#[test]
fn the_dispatcher_notes_every_load_and_answers_the_outcome_op() {
    let d: String = include_str!("../../../capsule_installer/src/server/dispatch.rs")
        .split_whitespace()
        .collect();
    assert!(d.contains("OP_LOAD_BY_NAME=>handlers::load_noted(req,sender.pid,kept),"));
    assert!(d.contains("OP_LOAD_OUTCOME=>handlers::load_outcome(req,sender.pid,kept),"));
}
