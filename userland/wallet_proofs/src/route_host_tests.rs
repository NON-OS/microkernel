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

//! The RPC host handed over on failure, and the last read each network shows.

/* A host that fails leaves its place to the next on its network, once,
 * however many requests failed on it, and the other network keeps its own. */
#[test]
fn a_failed_rpc_host_hands_over_to_the_next_on_its_network() {
    use crate::wallet::chain::{pick, rpc_failed, rpc_host, MAINNET, SEPOLIA};
    let _held = super::fetch_tests::hold();
    /* Start from the first host, whatever an earlier test left. */
    pick(false);
    while rpc_host() != MAINNET.rpcs[0] {
        rpc_failed(rpc_host());
    }
    pick(true);
    while rpc_host() != SEPOLIA.rpcs[0] {
        rpc_failed(rpc_host());
    }
    pick(false);
    let first = rpc_host();
    assert_eq!(first, MAINNET.rpcs[0]);
    assert!(rpc_failed(first));
    assert!(!rpc_failed(first), "a second failure on the old host moves nothing");
    assert_eq!(rpc_host(), MAINNET.rpcs[1]);
    pick(true);
    assert_eq!(rpc_host(), SEPOLIA.rpcs[0]);
    pick(false);
    for _ in 0..MAINNET.rpcs.len() - 1 {
        assert!(rpc_failed(rpc_host()));
    }
    assert_eq!(rpc_host(), MAINNET.rpcs[0], "round to the first again");
}

/* What Home and Settings say of each network's last read. */
#[test]
fn the_last_read_names_its_block_host_route_and_age() {
    use crate::last_read::{read_line, LastRead};
    let r = LastRead {
        block: 23_456_789,
        host: "ethereum-rpc.publicnode.com",
        route: "Nym",
        at_ms: 1_000,
    };
    assert_eq!(
        read_line(Some(r), 13_000),
        "block 23,456,789 from ethereum-rpc.publicnode.com over Nym, 12 s ago"
    );
    assert_eq!(
        read_line(Some(r), 1_000 + 125_000),
        "block 23,456,789 from ethereum-rpc.publicnode.com over Nym, 2 min ago"
    );
    assert_eq!(read_line(None, 0), "not read since this boot");
    let small = LastRead { block: 999, ..r };
    assert!(read_line(Some(small), 1_000).starts_with("block 999 from"));
}
