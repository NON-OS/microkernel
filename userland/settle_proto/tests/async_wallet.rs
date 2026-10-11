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

//! The wallet asks ASYNC for BROADCAST, READ and INTENT and nothing else
//! (v1.4.53); settle answers any other op it sends ASYNC with EINVAL.

use nonos_settle_proto::*;

const WALLET: u32 = 50;
const WALLET_2: u32 = 52;

fn owner(name: &[u8]) -> Option<u32> {
    match name {
        b"app.nonos_wallet" => Some(WALLET),
        b"app.nonos_wallet.2" => Some(WALLET_2),
        b"payment" => Some(41),
        _ => None,
    }
}

#[test]
fn the_wallet_may_ask_async_for_broadcast_gas_read_and_intent_only() {
    for op in [OP_BROADCAST, OP_GAS, OP_READ, OP_INTENT] {
        assert!(admits_flags(FLAG_ASYNC, op, WALLET, owner), "op {op}");
        assert!(admits_flags(FLAG_ASYNC, op, WALLET_2, owner), "op {op}");
    }
    for op in [OP_SETTLE, OP_DELIVERED, OP_FETCH, OP_LISTING, OP_OPENED, 3, 0, 99] {
        assert!(!admits_flags(FLAG_ASYNC, op, WALLET, owner), "op {op}");
    }
    assert!(admits_flags(0, OP_SETTLE, WALLET, owner), "a plain call is the op table's business");
    assert!(admits_flags(FLAG_ASYNC, OP_DELIVERED, 41, owner), "payment keeps every op");
    assert!(!admits_flags(FLAG_ASYNC, OP_READ, 0, |_| Some(0)), "no sender");
    assert!(!admits_flags(FLAG_ASYNC, OP_READ, 77, owner), "a pid holding no wallet name");
}
