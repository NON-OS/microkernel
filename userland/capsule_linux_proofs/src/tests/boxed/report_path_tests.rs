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

//! The report's path: every exit before the guest starts reports, every
//! refused write in the made trees counts, and the report is waited onto a
//! full inbox rather than dropped.

macro_rules! src {
    ($p:literal) => {
        include_str!(concat!("../../../../capsule_linux/src/linux/", $p))
    };
}

#[test]
fn every_exit_before_the_guest_starts_reports() {
    let start = src!("start.rs");
    assert_eq!(start.matches("super::start_stop::stop(").count(), 4);
    assert!(!start.contains("mk_exit(1)") && !start.contains("mk_exit(2)"));
    assert!(src!("start_stop.rs").contains("super::boxed::not_started(why, code);"));
}

#[test]
fn every_write_the_made_trees_refuse_counts() {
    assert!(src!("file/made/synth/refused_write.rs").contains("Kind::Write"));
    for (name, text) in [
        ("mknod", src!("file/mknod.rs")),
        ("perms", src!("file/meta/perms.rs")),
        ("stamp", src!("file/owner/stamp.rs")),
        ("rename", src!("file/rename/call.rs")),
        ("unlink", src!("file/dirops/unlink.rs")),
        ("rmdir", src!("file/dirops/dirs/remove.rs")),
        ("mkdir", src!("file/dirops/dirs/make.rs")),
    ] {
        assert!(text.contains("synth::refused_write("), "{name} refuses uncounted");
        assert!(!text.contains("synth::owns("), "{name} still asks owns");
    }
    assert_eq!(src!("file/made/synth_ops/open.rs").matches("Kind::Write").count(), 2);
}

#[test]
fn the_report_is_waited_onto_a_full_inbox() {
    let finish = src!("boxed/finish.rs");
    assert!(finish.contains("while mk_private_write(f) == -EBUSY && !until.expired()"));
}
