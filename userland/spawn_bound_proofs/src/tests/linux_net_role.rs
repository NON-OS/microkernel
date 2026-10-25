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

//! The terminal's network role asks for Network and nothing else, and the
//! tool bound checks a caller against that same mask: the bound's request is
//! the role's own, read from it, never a second literal that could drift.

use super::source::code_of;

const ROLE: &str = include_str!("../../../../src/userspace/capsule_linux/role_term_net.rs");
const MOD: &str = include_str!("../../../../src/userspace/capsule_linux/mod.rs");
const RUN: &str = include_str!("../../../../src/userspace/tool_capsules/run.rs");
const SPAWN: &str = include_str!("../../../../src/userspace/capsule_linux/role_spawn.rs");

fn flat(src: &str) -> String {
    code_of(src).split_whitespace().collect()
}

#[test]
fn the_network_role_asks_for_network_alone() {
    let role = flat(ROLE);
    let at = role.find("pub(super)constTERMINAL_NET:Role=Role{").expect("the role");
    let body = &role[at..at + role[at..].find("};").expect("its end")];
    assert!(body.contains("extra_caps:Capability::Network.bit(),"), "{body}");
    assert_eq!(body.matches("Capability::").count(), 1, "no second capability: {body}");
    assert!(body.contains("port:5188,") && body.contains("reply_port:5189,"));
}

#[test]
fn the_bound_asks_what_the_role_s_spawn_installs() {
    let extra = "pubconstTERMINAL_NET_EXTRA:u64=role_term_net::TERMINAL_NET.extra_caps;";
    assert!(flat(MOD).contains(extra));
    let run = flat(RUN);
    assert!(run.contains("letnet=crate::userspace::capsule_linux::TERMINAL_NET_EXTRA;"));
    assert!(run.contains("super::bound::check(LINUX_MANIFEST_BYTES,LINUX_CAPS|net,name)?;"));
    assert!(flat(SPAWN).contains("requested_caps:LINUX_CAPS|role.extra_caps,"), "the spawn's ask");
}
