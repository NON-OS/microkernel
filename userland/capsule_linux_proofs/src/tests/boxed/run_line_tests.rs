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

//! An unboxed run's closing line: silent when nothing was refused, every
//! refused kind counted as the box counts it, nothing naming a process.

use crate::boxed::kind::{Kind, ALL};
use crate::boxed::run_line::run_line;
use crate::boxed::tally::Tally;

#[test]
fn a_run_refused_nothing_leaves_no_line() {
    assert_eq!(run_line(&Tally::default()), None);
}

#[test]
fn the_line_counts_each_refused_kind_and_leaves_out_the_rest() {
    let mut t = Tally::default();
    for _ in 0..2 {
        t.call_begins();
        t.note(Kind::Write, b"/etc/passwd");
        t.note(Kind::Write, b"/etc/shadow");
    }
    t.call_begins();
    t.note(Kind::Network, b"SOCK_RAW");
    let line = run_line(&t).expect("refused");
    assert_eq!(line, "[LINUX] its policy refused 3: write=2 network=1\n");
    assert!(!line.contains("/etc"), "examples stay out of the line");
}

#[test]
fn every_kind_has_its_place_in_the_order_kind_rs_names_them() {
    let mut t = Tally::default();
    for kind in ALL.iter().rev() {
        t.call_begins();
        t.note(*kind, b"x");
    }
    let mut want = alloc::string::String::from("[LINUX] its policy refused 9:");
    for kind in ALL {
        want.push_str(&alloc::format!(" {}=1", kind.tag()));
    }
    want.push('\n');
    assert_eq!(run_line(&t).as_deref(), Some(&want[..]));
}
