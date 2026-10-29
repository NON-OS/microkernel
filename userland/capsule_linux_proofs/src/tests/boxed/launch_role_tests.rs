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
//! its guest, so nothing is done twice and thrown away; and what it proved
//! there is taken by the start, for those very bytes alone.

#[path = "../../../../capsule_linux/src/linux/start_guest/kept_rule.rs"]
mod kept_rule;

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
    let (proved, narrowed, kept) = (
        body.find("start_guest::prove(path, bytes, &launch.origin)").expect("not proved"),
        body.find("boxed::narrow(app)").expect("not narrowed"),
        body.find("start_guest::keep(path, bytes, app)").expect("not kept"),
    );
    assert!(proved < narrowed && narrowed < kept);
    assert!(
        body.contains("net_step(app.mask, mk_cap_check(me, NETWORK), false) == NetStep::AskAgain")
    );
    assert!(body.contains("&& mk_linux_run_net(me) == 0"));
    assert!(body.contains("    {\n        again()\n    }"), "early starts again one way");
    assert!(RUN_NET.contains("NetStep::AskAgain if mk_linux_run_net(me) == 0 => again(),"));
}

#[test]
fn a_kept_identity_is_for_the_same_path_and_the_very_bytes() {
    let (path, bytes) = (&b"/usr/bin/telegram-desktop"[..], vec![7u8; 64]);
    let kept = (path, bytes.as_ptr() as usize, bytes.len());
    assert!(kept_rule::same(kept, path, &bytes));
    assert!(!kept_rule::same(kept, b"/usr/bin/other", &bytes), "another path");
    let copy = bytes.clone();
    assert!(!kept_rule::same(kept, path, &copy), "the same bytes elsewhere are proved again");
    assert!(!kept_rule::same(kept, path, &bytes[..63]), "a shorter view of them is not them");
}
