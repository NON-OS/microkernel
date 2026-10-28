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

//! I-C6 with GTK's tools: their guests never share the program's family.

/*
 * A spawn joins the family of the guests its supervisor already has
 * (src/process/foreign/seal/open.rs, reserve), a start marks that family
 * started, and slot 0 is refused once it is (seal/sys_note.rs). A tool
 * spawned beside the program's guest so left a GTK program's run unnoted
 * and unkept: KeePassXC's serial said "the seal refused the app's note".
 * Each tool's guest is ended before the next spawn (serve/family_reap_end.rs),
 * so its family ends with it and the program's is a fresh one.
 */
#[test]
fn gtk_tools_run_before_the_programs_guest_is_made() {
    let start = include_str!("../../../../capsule_linux/src/linux/start.rs");
    let at = |needle: &str| start.find(needle).unwrap_or_else(|| panic!("{needle} is gone"));
    let (private, tools, spawned, started) = (
        at("super::file::prepare_private()"),
        at("super::gtk_prep::prepare(&launch.bytes);"),
        at("let pid = mk_foreign_spawn(b\"linux\");"),
        at("start(&mut guest, launch)"),
    );
    assert!(private < tools && tools < spawned && spawned < started);
    assert_eq!(start.matches("gtk_prep::prepare(").count(), 1);
}

#[test]
fn a_run_with_no_guest_leaves_no_private_directories() {
    let stop = include_str!("../../../../capsule_linux/src/linux/start_stop.rs");
    let body = &stop[stop.find("fn no_guest(").expect("no_guest is gone")..];
    let body = &body[..body.find("\n}\n").expect("no_guest ends")];
    assert!(body.find("clear_private()").unwrap() < body.find("stop(").unwrap());
}
