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

//! Every grant a refusal names is its own kind, and each kind its own tag.

use crate::boxed::kind::{of_needs, Kind, ALL};

#[test]
fn every_grant_a_refusal_names_is_its_own_kind() {
    for (needs, kind) in [
        ("EXEC", Kind::Exec),
        ("JIT", Kind::Jit),
        ("WAYLAND", Kind::Display),
        ("MODELS_RO", Kind::Models),
        ("NET_ANON", Kind::Network),
        ("NET_LISTEN_LO", Kind::Network),
        ("NET_ANON or NET_LISTEN_LO", Kind::Network),
        ("this destination", Kind::Network),
    ] {
        assert_eq!(of_needs(needs), kind, "{needs}");
    }
    let tags: Vec<_> = ALL.iter().map(|k| k.tag()).collect();
    let mut once = tags.clone();
    once.dedup();
    assert_eq!(tags, once);
    assert!(ALL.iter().enumerate().all(|(i, k)| k.index() == i));
}

#[test]
fn every_grant_a_refusal_names_has_its_kind() {
    let callers = [
        include_str!("../../../../capsule_linux/src/linux/file/models/open.rs"),
        include_str!("../../../../capsule_linux/src/linux/net/named/connect.rs"),
        include_str!("../../../../capsule_linux/src/linux/net/listen.rs"),
        include_str!("../../../../capsule_linux/src/linux/net/connect/out.rs"),
        include_str!("../../../../capsule_linux/src/linux/net/bind.rs"),
        include_str!("../../../../capsule_linux/src/linux/net/socket.rs"),
        include_str!("../../../../capsule_linux/src/linux/call/spawn/exec.rs"),
        include_str!("../../../../capsule_linux/src/linux/call/mem/map_anon.rs"),
        include_str!("../../../../capsule_linux/src/linux/call/mem/prot.rs"),
    ];
    let named =
        ["EXEC", "JIT", "WAYLAND", "MODELS_RO", "NET_ANON", "NET_LISTEN_LO", "this destination"];
    for text in callers {
        assert!(text.contains("app::refuse("));
        assert!(named.iter().any(|n| text.contains(&format!("\"{n}"))), "a grant no kind names");
    }
}
