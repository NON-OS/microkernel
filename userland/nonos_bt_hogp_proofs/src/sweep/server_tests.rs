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

//! E-64: the GATT server this host answers before pairing, under a seeded
//! client. Any opcode, any parameters: the reply is well formed — an Error
//! Response that echoes the request, or the matching response opcode — it
//! never runs past the ATT_MTU, and it never panics (overflow-checked).

use super::rng::each;
use nonos_bt_hogp::server::{serve, ATT_MTU};
use nonos_bt_wire::att::opcode::ERROR_RSP;

/// The opcodes the server answers, plus room for random and reserved ones.
const OPS: [u8; 9] = [0x02, 0x04, 0x06, 0x08, 0x0a, 0x10, 0x12, 0x00, 0xff];

#[test]
fn every_request_is_answered_within_the_mtu() {
    each(|rng| {
        let op = {
            let pool = [OPS[rng.below(OPS.len())], rng.byte(), rng.byte()];
            pool[rng.below(pool.len())]
        };
        let params = rng.span(0, 40);
        let r = serve(op, &params);
        assert!(!r.is_empty(), "op {op:#x}: an empty reply");
        assert!(r.len() <= ATT_MTU, "op {op:#x}: reply {} over the ATT_MTU", r.len());
        assert!(
            r[0] == ERROR_RSP || r[0] == op.wrapping_add(1),
            "op {op:#x}: reply opcode {:#x}",
            r[0]
        );
        if r[0] == ERROR_RSP {
            assert_eq!(r.len(), 5, "op {op:#x}: an error response is five octets");
            assert_eq!(r[1], op, "op {op:#x}: the error echoes the request opcode");
            assert_ne!(r[4], 0, "op {op:#x}: an error carries a reason");
        }
    });
}
