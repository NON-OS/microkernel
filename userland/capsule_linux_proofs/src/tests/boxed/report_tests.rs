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

//! The report the demo reads, word for word, and its serial line.

use crate::boxed::hash::box_hash;
use crate::boxed::kind::Kind;
use crate::boxed::line::box_line;
use crate::boxed::preset::parse;
use crate::boxed::report::{hex, report, End, SOURCE};
use crate::boxed::tally::Tally;
use nonos_linux_policy::mask::{EXEC, TTY};

fn ran(exit: i32) -> End<'static> {
    End { exit, not_started: None }
}

fn demo() -> Tally {
    // A script writes its scratch folder (allowed, nothing counted), writes
    // outside it, and opens a connection.
    let mut t = Tally::default();
    t.call_begins();
    t.note(Kind::Write, b"/home/out.txt");
    t.call_begins();
    t.note(Kind::Network, b"an internet socket");
    t
}

#[test]
fn the_demo_reports_exactly_its_two_refusals() {
    let p = parse(b"strict").unwrap();
    let h = box_hash(&p);
    let want = format!(
        "BOX REPORT v1\n  policy   strict {}\n  allowed  write /tmp\n  allowed  read the image, read-only\n  allowed  network none\n  allowed  execve, each program held to this box\n  refused  2\n    write         1  /home/out.txt\n    network       1  an internet socket\n  exit     0\n  {SOURCE}\n",
        hex(&h)
    );
    assert_eq!(report(&p, &h, &demo(), Some(TTY | EXEC), &ran(0)), want);
}

#[test]
fn the_serial_line_carries_every_kind_and_the_exit() {
    let p = parse(b"strict").unwrap();
    let h = box_hash(&p);
    let want = format!(
        "[BOX] v=1 preset=strict policy={} refused=2 write=1 network=1 exec=0 jit=0 display=0 models=0 call=0 barred=0 unserved=0 started=yes exit=3 source=capsule_linux\n",
        hex(&h)
    );
    assert_eq!(box_line(&p, &h, &demo(), &ran(3)), want);
}

#[test]
fn a_clean_run_says_it_was_refused_nothing() {
    let p = parse(b"allow:/tmp/out,net:10.0.0.1:443").unwrap();
    let text = report(&p, &box_hash(&p), &Tally::default(), None, &ran(0));
    assert!(text.contains("  refused  0\n  exit     0\n"), "{text}");
    assert!(text.contains("  allowed  write /tmp/out\n"), "{text}");
    assert!(text.contains("  allowed  network 10.0.0.1:443\n"), "{text}");
}
