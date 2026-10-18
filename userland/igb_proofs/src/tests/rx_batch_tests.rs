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

//! net.core asks every wired card for OP_RX_BATCH first and takes a reply
//! with status 0 as a batch of frames. igb answered that op with its
//! register snapshot, so every frame an igb port received was dropped (the
//! the maintainers' review). The counters now sit past it, and the op itself
//! reaches the server's refusal for an op it does not serve.

use crate::protocol::OP_STATS;

fn net_cores_batch_op() -> u16 {
    include_str!("../../../capsule_net_core/src/protocol/ops.rs")
        .lines()
        .find_map(|l| l.strip_prefix("pub const OP_RX_BATCH: u16 = "))
        .and_then(|v| v.trim_end_matches(';').parse::<u16>().ok())
        .expect("net.core names its batch op")
}

#[test]
fn the_counters_are_not_on_net_cores_batch_op() {
    assert_ne!(OP_STATS, net_cores_batch_op());
}

#[test]
fn the_batch_op_falls_to_the_servers_refusal() {
    let runner = include_str!("../../../capsule_driver_igb/src/server/runner.rs");
    // Every op the server serves is named in its match; anything else is
    // answered E_INVAL, which net.core reads as no batch support.
    assert!(runner.contains("_ => reply_with_status(sender, &mut tx, &req, E_INVAL)"));
    assert!(!runner.contains("OP_RX_BATCH"));
}
