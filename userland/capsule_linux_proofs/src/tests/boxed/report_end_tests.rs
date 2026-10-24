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

//! A run that never started says so, and what a run was allowed is what the
//! program got in the box, not the box as named.

use crate::boxed::hash::box_hash;
use crate::boxed::line::box_line;
use crate::boxed::preset::parse;
use crate::boxed::report::{report, End, SOURCE};
use crate::boxed::tally::Tally;
use nonos_linux_policy::mask::{EXEC, FILES_HOME, NET_ANON, TTY};

#[test]
fn a_run_that_never_started_says_why_and_is_never_a_clean_run() {
    let p = parse(b"strict").unwrap();
    let e = End { exit: 2, not_started: Some("unproven program") };
    let t = Tally::default();
    let text = report(&p, &box_hash(&p), &t, None, &e);
    assert!(text.contains("  started  no: unproven program\n  exit     2\n"), "{text}");
    assert!(box_line(&p, &box_hash(&p), &t, &e).contains(" started=no exit=2 "));
}

#[test]
fn allowed_is_what_the_program_got() {
    let p = parse(b"allow:/tmp/a,/home/b,net:10.0.0.1:443").unwrap();
    let h = box_hash(&p);
    let ran = End { exit: 0, not_started: None };
    let t = Tally::default();
    // A program with no home and no network of its own gets neither here.
    let got = report(&p, &h, &t, Some(TTY | EXEC), &ran);
    assert!(got.contains("  allowed  write /tmp/a\n"), "{got}");
    assert!(got.contains("  allowed  network none\n"), "{got}");
    // One granted both gets both.
    let all = report(&p, &h, &t, Some(TTY | EXEC | FILES_HOME | NET_ANON), &ran);
    assert!(all.contains("  allowed  write /home/b /tmp/a\n"), "{all}");
    assert!(all.contains("  allowed  network 10.0.0.1:443\n"), "{all}");
    // Nothing writable, and no execve, said as such.
    let bare = report(&parse(b"allow:/home/b").unwrap(), &h, &t, Some(TTY), &ran);
    assert!(
        bare.contains("  allowed  write nothing\n") && bare.contains("  allowed  execve none\n")
    );
}

#[test]
fn the_report_says_whose_counts_these_are() {
    assert!(
        SOURCE.contains("capsule_linux") && SOURCE.contains("not the kernel's sealed histogram")
    );
}
