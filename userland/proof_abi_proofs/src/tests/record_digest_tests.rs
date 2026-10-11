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

use nonos_proof_abi::{ctx, record_id, Kind};

use crate::fixture::family;

#[test]
fn ctx_and_id_move_with_every_byte_and_ctx_with_the_kind() {
    let b = family().record.encode();
    let (c, id) = (ctx(Kind::Isolation, &b), record_id(&b));
    assert_ne!(ctx(Kind::Delivery, &b), c, "one record, two kinds, two contexts");
    for at in [6, 8, 56, 64, 96, 136, 176, 239] {
        let mut g = b;
        g[at] ^= 1;
        assert_ne!(ctx(Kind::Isolation, &g), c, "ctx at {at}");
        assert_ne!(record_id(&g), id, "id at {at}");
    }
    assert!(c.iter().all(|&w| w < 0xFFFF_FFFF_0000_0001), "ctx words are field words");
}

#[test]
fn ctx_is_the_gates_digest_reading_over_kind_length_and_record() {
    let b = family().record.encode();
    let pre = [&b"NONOS-PROOF-RECORD-v1"[..], &1u16.to_le_bytes(), &240u32.to_le_bytes(), &b];
    let d = blake3::hash(&pre.concat());
    let want = nonos_device_attest::words_of(d.as_bytes()).map(|f| f.to_u64());
    assert_eq!(ctx(Kind::Isolation, &b), want);
    let id = blake3::derive_key("NONOS-PROOF-RECORD-ID-v1", &b);
    assert_eq!(record_id(&b), id);
}
