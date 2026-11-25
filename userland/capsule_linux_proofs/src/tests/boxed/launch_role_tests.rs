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

//! A launch decides its role before anything runs for it: a program whose
//! box has NET_ANON starts again in the network role before GTK's tools or
//! its guest, so nothing is done twice and thrown away. The role is chosen on
//! a binding alone; the program is proved by the start, after its family
//! exists, every time, and no proof is kept from before it.

const PROVE: &str = include_str!("../../../../capsule_linux/src/linux/start_guest/prove.rs");
const START: &str = include_str!("../../../../capsule_linux/src/linux/start.rs");
const RUN_NET: &str = include_str!("../../../../capsule_linux/src/linux/run_net.rs");

#[test]
fn the_role_is_decided_before_the_tools_and_the_guest() {
    let at = |needle: &str| START.find(needle).unwrap_or_else(|| panic!("{needle} is gone"));
    let (early, tools) =
        (at("super::run_net::early(&launch);"), at("gtk_prep::prepare(&launch.bytes)"));
    let spawned = at("let pid = mk_foreign_spawn(b\"linux\");");
    assert!(at("super::file::prepare_private()") < early && early < tools && tools < spawned);
}

#[test]
fn early_asks_again_only_for_net_anon_in_a_role_without_network() {
    let body = &RUN_NET[RUN_NET.find("pub(super) fn early(").expect("early is gone")..];
    let body = &body[..body.find("\n}\n").expect("early ends")];
    let (bound, narrowed) = (
        body.find("start_guest::bind_role(path, bytes, &launch.origin)").expect("not bound"),
        body.find("boxed::narrow(app)").expect("not narrowed"),
    );
    assert!(bound < narrowed);
    assert!(!body.contains("prove(") && !body.contains("keep("), "nothing proved or kept early");
    assert!(
        body.contains("net_step(app.mask, mk_cap_check(me, NETWORK), false) == NetStep::AskAgain")
    );
    assert!(body.contains("&& mk_linux_run_net(me) == 0"));
    assert!(body.contains("    {\n        again()\n    }"), "early starts again one way");
    assert!(RUN_NET.contains("NetStep::AskAgain if mk_linux_run_net(me) == 0 => again(),"));
}

#[test]
fn the_start_proves_every_time_and_keeps_nothing() {
    let code: String = PROVE.lines().filter(|l| !l.trim_start().starts_with("//")).collect();
    for gone in ["KEPT", "kept_rule", "fn keep(", "take_if"] {
        assert!(!code.contains(gone), "{gone} is back");
    }
    assert!(code.contains("Origin::Store => super::super::attest::verify(path, bytes).and_then(|_| bind(path, bytes)),"));
}
