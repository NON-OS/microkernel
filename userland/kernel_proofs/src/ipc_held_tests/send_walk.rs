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

//! A whole addressed send, walked gate by gate in the order
//! send_address.rs as_addressed takes them, each by the rule its kernel gate
//! decides by (registry/send_rule.rs), down to the route's own check in
//! ipc/kernel_ipc.rs. Each gate passing its own tests is not enough: a send
//! is admitted only when all of them agree. Before the route applied
//! caps_admit, the Linux personality's grant request to net.socks5 passed
//! the three gates before it and was refused there, with nothing said.
//!
//! And no capability test of an endpoint's requirement on a send path is
//! made by any other rule: every file under src/ipc and
//! src/syscall/microkernel/ipc is read as it stands.

use std::fs;
use std::path::{Path, PathBuf};

use crate::capabilities::{bit_of, Capability};
use crate::service_policy::required_caps;

use super::common::{owner, Gate, IPC};
use super::foreign_fronts::{personality, served};
use super::registry_addressed::addressed_admits;
use super::send_rule::{endpoint_caps, reaches, routes, writes_inbox};

/// What the kernel knows of a send when it decides it.
struct Send {
    target: &'static str,
    /// The process serving `target`, read from its one inbox.
    serves: [Gate; 2],
    /// The caller's capability token, which the gates read.
    held: u64,
    /// What the kernel keeps for the caller's pid, which the route reads;
    /// None once the process has ended.
    pid_held: Option<u64>,
    /// The calling process's name.
    caller: Option<&'static str>,
    /// The endpoints the caller owns.
    owns: &'static [&'static str],
}

/// The first gate to refuse `s`, named as the kernel function holding it,
/// or Ok when the message is queued.
fn walk(s: &Send) -> Result<(), &'static str> {
    let required = s.serves.iter().find(|g| g.0 == s.target).map_or(0, |g| g.1);
    let held_to = s.serves.iter().find(|g| g.0 == s.target).and_then(|g| g.2);
    if !endpoint_caps(s.target, required, s.held) {
        return Err("caller_satisfies_endpoint");
    }
    if !reaches(s.target, held_to, s.caller, owner(s.owns)) {
        return Err("caller_may_reach");
    }
    let inbox_ok = || writes_inbox(s.serves, s.held, owner(s.owns));
    if !addressed_admits(false, owner(s.owns)(s.target), inbox_ok) {
        return Err("caller_may_address");
    }
    if !routes(s.target, required, s.pid_held) {
        return Err("kernel_route_ipc_corr");
    }
    Ok(())
}

/// The personality's send to `target`, whose process also serves `reply`.
fn from_personality(target: &'static str, reply: &'static str, held: u64) -> Send {
    Send {
        target,
        serves: served(target, reply),
        held,
        pid_held: Some(held),
        caller: Some("app.linux.run"),
        owns: &["app.linux.run", "endpoint.app.linux.run.reply"],
    }
}

const FRONTS: [(&str, &str); 2] =
    [("net.socks5", "endpoint.net.socks5.reply"), ("net.anon", "endpoint.net.anon.reply")];

const OTHERS: [(&str, &str); 3] = [
    ("net.sockets", "endpoint.net.sockets.reply"),
    ("net.core", "endpoint.net.core.reply"),
    ("net.nym", "endpoint.net.nym.reply"),
];

#[test]
fn the_personality_s_grant_request_to_net_socks5_is_queued() {
    for (front, reply) in FRONTS {
        assert_eq!(walk(&from_personality(front, reply, personality())), Ok(()), "{front}");
    }
}

#[test]
fn and_its_send_to_any_other_network_service_is_refused_at_the_first_gate() {
    for (name, reply) in OTHERS {
        let s = from_personality(name, reply, personality());
        assert_eq!(walk(&s), Err("caller_satisfies_endpoint"), "{name}");
    }
}

#[test]
fn a_caller_without_foreign_exec_reaches_no_front() {
    let plain = personality() & !bit_of(Capability::ForeignExec);
    for (front, reply) in FRONTS {
        let s = from_personality(front, reply, plain);
        assert_eq!(walk(&s), Err("caller_satisfies_endpoint"), "{front}");
    }
}

#[test]
fn the_route_still_refuses_what_the_pid_does_not_hold() {
    // The token admits, the pid's own bits do not: the route is a check of
    // its own, by the same rule.
    let mut s = from_personality("net.socks5", "endpoint.net.socks5.reply", personality());
    s.pid_held = Some(personality() & !bit_of(Capability::ForeignExec));
    assert_eq!(walk(&s), Err("kernel_route_ipc_corr"));
    s.pid_held = None;
    assert_eq!(walk(&s), Err("kernel_route_ipc_corr"), "an ended process");
}

#[test]
fn a_network_holder_is_queued_as_before() {
    let network = IPC | bit_of(Capability::Network);
    for (name, reply) in FRONTS.iter().chain(OTHERS.iter()) {
        let s = Send {
            target: name,
            serves: [(name, required_caps(name, IPC), None), (reply, IPC, None)],
            held: network,
            pid_held: Some(network),
            caller: Some("app.browser"),
            owns: &["app.browser"],
        };
        assert_eq!(walk(&s), Ok(()), "{name}");
    }
}

/// as_addressed takes the gates in the walk's order, and each kernel gate
/// decides by the rule the walk calls.
#[test]
fn the_kernel_takes_the_gates_in_this_order_by_these_rules() {
    let address = include_str!("../../../../src/syscall/microkernel/ipc/send_address.rs");
    let order = [
        "caller_satisfies_endpoint(endpoint, target)",
        "caller_may_reach(target)",
        "caller_may_address(target)",
        "kernel_route_ipc_corr(pid, target, data, correlation)",
    ];
    let at: Vec<usize> = order.iter().map(|c| address.find(c).expect(c)).collect();
    assert!(at.windows(2).all(|w| w[0] < w[1]), "as_addressed's order");
    assert!(address.contains("caller_may_write_inbox(ep.pid, held)"));

    let caps = include_str!("../../../../src/syscall/microkernel/ipc/send_caps.rs");
    let reach = include_str!("../../../../src/services/registry/peers_check.rs");
    let inbox = include_str!("../../../../src/services/registry/peers_inbox.rs");
    let route = include_str!("../../../../src/ipc/kernel_ipc.rs");
    assert!(caps.contains("if endpoint_caps(target, required, held) {"));
    assert!(reach.contains("if reaches(target, held_to.as_deref(), caller_name().as_deref(), caller_owns) {"));
    assert!(inbox.contains("if writes_inbox(gates, held, caller_owns) {"));
    assert!(route.contains("if routes(target, endpoint.caps_required, held) {"));
    assert_eq!(route.matches("if !route_admits(caller_pid, target, &endpoint) {").count(), 2);
}

fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(dir).expect("a send path reads") {
        let p = e.expect("an entry").path();
        if p.is_dir() {
            sources(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

/// The ways the plain rule was written against an endpoint's requirement.
fn plain_rule(line: &str) -> bool {
    let code = line.split("//").next().unwrap_or("");
    (code.contains("has(") && code.contains("caps_required"))
        || code.contains("& required == required")
        || code.contains("&required == required")
        || code.contains("& endpoint.caps_required")
        || code.contains("& ep.caps_required")
        || code.contains("caps_required ==")
}

#[test]
fn no_send_path_tests_an_endpoint_requirement_by_another_rule() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src");
    let mut files = Vec::new();
    sources(&src.join("ipc"), &mut files);
    sources(&src.join("syscall/microkernel/ipc"), &mut files);
    assert!(files.len() > 80, "both trees are read: {}", files.len());
    for f in files {
        let text = fs::read_to_string(&f).expect("a source reads");
        for (n, line) in text.lines().enumerate() {
            assert!(!plain_rule(line), "{}:{} {}", f.display(), n + 1, line.trim());
        }
    }
    // The pattern finds what the route held before.
    assert!(plain_rule("    if !caps::has(caller_pid, endpoint.caps_required) {"));
    assert!(plain_rule("    if held & required == required {"));
}
