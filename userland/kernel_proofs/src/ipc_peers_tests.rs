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

//! The IPC peer list, run as the kernel runs it: a capsule held to a list
//! reaches its peers and nothing else, and a capsule off the list is left
//! as it was. Deleting the prover's row makes `prover_reaches_only_core` fail.

#[path = "../../../src/services/registry/peers.rs"]
mod peers;

use peers::may_reach;

#[test]
fn prover_reaches_only_core() {
    assert!(may_reach("shield_prover", "shield.core"));
    for other in ["net.sockets", "net.tcp", "net.nym", "vfs", "wallet", "shield.net"] {
        assert!(!may_reach("shield_prover", other), "the prover reached {other}");
    }
}

#[test]
fn an_unlisted_capsule_is_unaffected() {
    assert!(may_reach("vfs", "net.sockets"));
    assert!(may_reach("shield_core", "shield.prover"));
}
