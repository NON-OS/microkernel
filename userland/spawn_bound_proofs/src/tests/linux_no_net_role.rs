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

//! No Linux role but install holds Network. A run's guests reach the
//! anonymity network on the grant a front makes for their family
//! (userland/capsule_linux/design/network-grant.md), so the run and terminal
//! roles ask for nothing beyond LINUX_CAPS, and there is no role, tool or
//! ticket that starts the personality again with Network.

use super::source::code_of;

const ROLES: &str = include_str!("../../../../src/userspace/capsule_linux/roles.rs");
const SLOTS: &str = include_str!("../../../../src/userspace/capsule_linux/terminal/roles.rs");
const MOD: &str = include_str!("../../../../src/userspace/capsule_linux/mod.rs");
const RUN: &str = include_str!("../../../../src/userspace/tool_capsules/run.rs");
const REGISTRY: &str = include_str!("../../../../src/userspace/tool_capsules/registry.rs");

fn flat(src: &str) -> String {
    code_of(src).split_whitespace().collect()
}

#[test]
fn only_the_install_role_asks_for_network() {
    let roles = flat(ROLES);
    let install = roles.find("constINSTALL:Role").expect("the install role");
    let end = install + roles[install..].find("};").expect("its end");
    assert_eq!(roles.matches("Capability::Network").count(), 1, "named once in roles.rs");
    let named = roles.find("Capability::Network").unwrap();
    assert!(install < named && named < end, "and that once, in INSTALL");
    for role in ["constRUN:Role", "constTERMINAL:Role"] {
        let at = roles.find(role).expect(role);
        let body = &roles[at..at + roles[at..].find("};").unwrap()];
        assert!(body.contains("extra_caps:0,"), "{role} asks for nothing more");
    }
    assert!(!flat(SLOTS).contains("Capability::Network"), "no tier slot holds Network");
}

#[test]
fn no_network_role_tool_or_ticket_is_left() {
    let m = flat(MOD);
    for gone in ["modrole_term_net;", "modrun_net;", "modterm_ticket;", "spawn_terminal_net", "spawn_run_net"] {
        assert!(!m.contains(gone), "{gone} is back");
    }
    assert!(!flat(RUN).contains("LINUX_NET_TOOL") && !flat(REGISTRY).contains("tool.linux.net"));
}
