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

//! Each entry's field list says what its type string says, member by
//! member, so a word is checked against the type the contract decodes it as.

use crate::server::typed::field::Kind;
use crate::server::typed::table::ENTRIES;

fn kind_of(solidity: &str) -> Kind {
    match solidity {
        "bytes32" => Kind::Bytes32,
        "address" => Kind::Address,
        u => Kind::Uint(u.strip_prefix("uint").unwrap().parse().unwrap()),
    }
}

#[test]
fn fields_follow_the_type_string() {
    for e in ENTRIES {
        let inner = e.typed.type_string.split_once('(').unwrap().1.strip_suffix(')').unwrap();
        let members: Vec<(&str, &str)> =
            inner.split(',').map(|m| m.split_once(' ').unwrap()).collect();
        assert_eq!(members.len(), e.fields.len(), "entry {}", e.id);
        assert_eq!(e.typed.fields, e.fields.len(), "entry {}", e.id);
        for ((ty, name), f) in members.iter().zip(e.fields) {
            assert_eq!(kind_of(ty), f.kind, "entry {} {name}", e.id);
            assert_eq!(f.is_signer, e.id == 1 && *name == "payer", "entry {} {name}", e.id);
        }
    }
}

#[test]
fn the_ids_are_the_plans() {
    let ids: Vec<u16> = ENTRIES.iter().map(|e| e.id).collect();
    assert_eq!(ids, [1, 2, 3]);
    assert_eq!(ENTRIES[0].callers, [b"payment".as_slice()]);
    for e in &ENTRIES[1..] {
        assert!(e.callers.iter().all(|c| c.starts_with(b"app.nonos_wallet")));
    }
}

/// NOX's own EIP-2612 permit signs under domain "NONOS"; an entry there
/// would let op 30 sign a permit (plan/31).
#[test]
fn no_entry_signs_under_the_tokens_permit_domain() {
    assert!(ENTRIES.iter().all(|e| e.typed.domain.name != "NONOS"));
}
