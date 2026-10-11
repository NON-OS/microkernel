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

use nonos_proof_abi::{ctx, record_id};

use crate::fixture::family;

#[test]
fn ctx_and_id_move_with_every_byte_and_differ_from_each_other() {
    let b = family().record.encode();
    let (c, id) = (ctx(&b), record_id(&b));
    assert_ne!(id, [0; 32]);
    for at in [8, 56, 64, 96, 136, 176, 239] {
        let mut g = b;
        g[at] ^= 1;
        assert_ne!(ctx(&g), c, "ctx at {at}");
        assert_ne!(record_id(&g), id, "id at {at}");
    }
    let p = 0xFFFF_FFFF_0000_0001u64;
    assert!(c.iter().all(|&w| w < p), "ctx words are field words");
}

#[test]
fn ctx_is_the_gates_digest_reading() {
    let b = family().record.encode();
    let d = blake3::hash(&[&b"NONOS-PROOF-RECORD-v1"[..], &240u32.to_le_bytes(), &b].concat());
    let want = nonos_device_attest::words_of(d.as_bytes()).map(|f| f.to_u64());
    assert_eq!(ctx(&b), want);
}
