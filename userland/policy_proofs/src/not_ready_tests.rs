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

//! Before the restore finishes, a kept field is answered "not ready", never
//! with the default the store holds until then; every other field, and
//! every field after it, as before.

use nonos_libc::take_replies;
use nonos_policy_proto::settings_record::KEPT;
use nonos_policy_proto::{
    decode_field, kind_of, Field, Header, E_NOT_READY, E_OK, HDR_LEN, OP_GET,
};

use crate::restore::set_settled;
use crate::server::serve::serve;

const ASKER: u32 = 41;

/// The status a read of `field` gets, after checking it is answered once,
/// to the asker, under the op and field it asked.
fn read(field: Field) -> u16 {
    let mut f = vec![0u8; HDR_LEN];
    let kind = kind_of(field);
    Header { op: OP_GET, field: field as u32, kind, status: 0, payload_len: 0 }.encode(&mut f);
    let _ = take_replies();
    serve(ASKER, &f);
    let mut replies = take_replies();
    assert_eq!(replies.len(), 1, "{:#x}: one reply", field as u32);
    let (to, reply) = replies.remove(0);
    assert_eq!(to, ASKER);
    let hdr = Header::decode(&reply).expect("a whole header");
    assert_eq!((hdr.op, hdr.field, hdr.kind), (OP_GET, field as u32, kind));
    hdr.status
}

#[test]
fn kept_fields_are_not_ready_before_the_restore() {
    set_settled(false);
    for field in KEPT.iter().copied().chain([Field::Persistent]) {
        assert_eq!(read(field), E_NOT_READY, "{:#x} answered before the restore", field as u32);
    }
}

#[test]
fn other_fields_answer_before_the_restore() {
    set_settled(false);
    let waits = |f: &Field| *f == Field::Persistent || KEPT.contains(f);
    let others: Vec<Field> =
        (0..=0xFFFFu32).filter_map(decode_field).filter(|f| !waits(f)).collect();
    assert!(others.len() > 10, "fields to try");
    for field in others {
        assert_ne!(read(field), E_NOT_READY, "{:#x} held for a restore it is not in", field as u32);
    }
}

#[test]
fn kept_fields_answer_once_the_restore_is_done() {
    set_settled(true);
    for field in KEPT.iter().copied().chain([Field::Persistent]) {
        assert_eq!(read(field), E_OK, "{:#x} after the restore", field as u32);
    }
}
