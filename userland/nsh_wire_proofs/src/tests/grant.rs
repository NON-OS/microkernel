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

//! A grant: every field placed, and each rule the bytes alone can settle
//! refused when broken.

use crate::build::grant::GrantB;
use nsh_wire::{Grant, GrantKind, WireError};

pub fn parse(b: &GrantB) -> Result<(), WireError> {
    Grant::parse(&b.bytes()).map(|_| ())
}

#[test]
fn a_base_grant_places_every_field() {
    let b = GrantB::default();
    let bytes = b.bytes();
    let g = Grant::parse(&bytes).expect("valid");
    assert_eq!((g.kind, g.grant_id, g.parent_id), (GrantKind::Base, &[0x11; 16], &[0; 16]));
    assert_eq!((g.fleet_id, g.subject, g.issuer), (&[0xF1; 16], &[0x5B; 64], &[0x15; 32]));
    assert_eq!((g.user, g.role), (&b"ek"[..], &b"auditor"[..]));
    assert_eq!(g.caps.iter().collect::<Vec<_>>(), [&b"CoreExec"[..], b"FileSystem", b"IPC"]);
    assert_eq!((g.not_before_ms, g.not_after_ms), (1_000, 2_000));
    assert_eq!(g.signed, &bytes[..bytes.len() - 3373 - 1]);
    assert_eq!(g.issued, &bytes[..bytes.len() - 1]);
    assert_eq!(g.approvals().count(), 0);
}

#[test]
fn the_head_is_judged() {
    let d = GrantB::default;
    assert_eq!(parse(&GrantB { magic: *b"NSHE", ..d() }), Err(WireError::Magic));
    assert_eq!(parse(&GrantB { version: 2, ..d() }), Err(WireError::BodyVersion));
    assert_eq!(parse(&GrantB { reserved: 1, ..d() }), Err(WireError::Reserved));
    for kind in [0, 3, 255] {
        assert_eq!(parse(&GrantB { kind, ..d() }), Err(WireError::Value), "{kind}");
        let up = GrantB { kind, ..GrantB::escalation() };
        assert_eq!(parse(&up), Err(WireError::Value), "{kind} with a parent");
    }
}

#[test]
fn ids_names_and_the_window_are_judged() {
    let d = GrantB::default;
    let bad = [
        GrantB { grant_id: [0; 16], ..d() },
        GrantB { fleet_id: [0; 16], ..d() },
        GrantB { user: b"Ek".to_vec(), ..d() },
        GrantB { role: b"Auditor".to_vec(), ..d() },
        GrantB { caps: vec![], ..d() },
        GrantB { caps: vec![b"ipc".to_vec()], ..d() },
        GrantB { not_before: 2_000, ..d() },
        GrantB { not_before: 2_001, ..d() },
    ];
    for (i, b) in bad.iter().enumerate() {
        assert_eq!(parse(b), Err(WireError::Value), "case {i}");
    }
    assert_eq!(parse(&GrantB { not_before: 1_999, ..d() }), Ok(()));
    assert_eq!(parse(&GrantB { user: vec![b'u'; 33], ..d() }), Err(WireError::Oversize));
    assert_eq!(parse(&GrantB { role: vec![], ..d() }), Err(WireError::Value));
}
