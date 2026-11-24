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

//! A ForeignExec holder without Network, the Linux personality, reaches the
//! two anonymity fronts by name and by pid, and no other network service by
//! either. A send by name passes the endpoint's capability gate and then the
//! gate of every endpoint its owner serves (send_address.rs); a send by pid
//! passes the second alone (send_caps_inbox.rs). Both apply one rule,
//! registry/policy.rs caps_admit. Before the inbox gate applied it, the
//! personality's grant request to net.socks5 passed the first gate and was
//! refused by the second.

use crate::capabilities::{bit_of, Capability};
use crate::service_policy::{caps_admit, required_caps};

use super::common::{holding, owner, Gate, IPC};
use super::held::inbox_admits;
use super::registry_addressed::addressed_admits;

/// The Linux personality's floor (src/userspace/capsule_linux/spawn.rs
/// LINUX_CAPS): ForeignExec and no Network.
fn personality() -> u64 {
    [
        Capability::CoreExec,
        Capability::IPC,
        Capability::Memory,
        Capability::Crypto,
        Capability::Debug,
        Capability::ForeignExec,
    ]
    .into_iter()
    .map(bit_of)
    .fold(0, |a, b| a | b)
}

/// What a network service's process serves: its name, with the requirement
/// the kernel registers for it, and its reply inbox, which needs IPC.
fn served(name: &'static str, reply: &'static str) -> [Gate; 2] {
    [(name, required_caps(name, IPC), None), (reply, IPC, None)]
}

const FRONTS: [(&str, &str); 2] =
    [("net.anon", "endpoint.net.anon.reply"), ("net.socks5", "endpoint.net.socks5.reply")];

const OTHERS: [(&str, &str); 3] = [
    ("net.sockets", "endpoint.net.sockets.reply"),
    ("net.core", "endpoint.net.core.reply"),
    ("net.nym", "endpoint.net.nym.reply"),
];

/// A send by pid into the process serving `gates`, from a caller holding
/// `held` and owning nothing.
fn by_pid(gates: [Gate; 2], held: u64) -> bool {
    let caps = |name: &str, required: u64| caps_admit(name, required, held);
    inbox_admits(gates, caps, owner(&[]))
}

/// A send by `name` to the process serving `gates`, as send_address.rs runs
/// it: the endpoint's own gate, then the inbox's.
fn by_name(name: &str, gates: [Gate; 2], held: u64) -> bool {
    let required = gates.iter().find(|g| g.0 == name).map_or(0, |g| g.1);
    caps_admit(name, required, held) && addressed_admits(false, false, || by_pid(gates, held))
}

#[test]
fn the_personality_reaches_each_front_by_name_and_by_pid() {
    for (front, reply) in FRONTS {
        let gates = served(front, reply);
        assert!(by_name(front, gates, personality()), "{front} by name");
        assert!(by_pid(gates, personality()), "{front} by pid");
    }
}

#[test]
fn and_no_other_network_service_by_either() {
    for (name, reply) in OTHERS {
        let gates = served(name, reply);
        assert!(!by_name(name, gates, personality()), "{name} by name");
        assert!(!by_name(reply, gates, personality()), "{name} by its reply endpoint");
        assert!(!by_pid(gates, personality()), "{name} by pid");
    }
}

#[test]
fn a_caller_without_foreign_exec_or_network_reaches_none_of_the_five() {
    let plain = personality() & !bit_of(Capability::ForeignExec);
    for (name, reply) in FRONTS.iter().chain(OTHERS.iter()) {
        let gates = served(name, reply);
        assert!(!by_name(name, gates, plain), "{name} by name");
        assert!(!by_pid(gates, plain), "{name} by pid");
    }
}

#[test]
fn the_exception_names_the_fronts_and_opens_nothing_else_they_might_serve() {
    // A front's process serving a second endpoint that needs Network and is
    // not a front is not reached through the exception, by pid or by name.
    let wider: [Gate; 2] = [
        ("net.socks5", required_caps("net.socks5", IPC), None),
        ("net.dns", required_caps("net.dns", IPC), None),
    ];
    assert!(!by_pid(wider, personality()));
    assert!(!by_name("net.socks5", wider, personality()));
    // The plain rule, which every other test of the inbox gate uses, admits
    // no ForeignExec holder anywhere it would not admit any caller.
    let gates = served("net.socks5", "endpoint.net.socks5.reply");
    assert!(!inbox_admits(gates, holding(personality()), owner(&[])));
}

#[test]
fn both_gates_apply_the_one_rule() {
    let send_caps = include_str!("../../../../src/syscall/microkernel/ipc/send_caps.rs");
    let peers_inbox = include_str!("../../../../src/services/registry/peers_inbox.rs");
    assert!(send_caps.contains("if caps_admit(target, required, held) {"));
    assert!(peers_inbox.contains("super::policy::caps_admit(name, required, held)"));
    assert!(peers_inbox.contains("if inbox_admits(gates, caps, caller_owns) {"));
}
