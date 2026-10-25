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

//! Each refusal counted once, under its one kind: never twice for one call,
//! never lost across calls, its examples kept as shown.

use crate::boxed::kind::{Kind, ALL};
use crate::boxed::tally::{Tally, EXAMPLES, EXAMPLE_LEN};

#[test]
fn each_kind_counts_its_own_refusals_and_no_other() {
    for kind in ALL {
        let mut t = Tally::default();
        for n in 1..=5u64 {
            t.call_begins();
            t.note(kind, b"x");
            assert_eq!(t.count(kind), n, "{}", kind.tag());
        }
        for other in ALL.iter().filter(|k| **k != kind) {
            assert_eq!(t.count(*other), 0, "{} counted as {}", kind.tag(), other.tag());
        }
        assert_eq!(t.total(), 5);
    }
}

#[test]
fn one_call_refused_twice_as_one_kind_is_one_refusal() {
    let mut t = Tally::default();
    t.call_begins();
    t.note(Kind::Write, b"/etc/a");
    t.note(Kind::Write, b"/etc/b");
    assert_eq!(t.count(Kind::Write), 1);
    assert_eq!(t.examples(Kind::Write), [b"/etc/a".to_vec()]);
    t.note(Kind::Network, b"socket");
    assert_eq!((t.count(Kind::Network), t.total()), (1, 2));
    t.call_begins();
    t.note(Kind::Write, b"/etc/c");
    assert_eq!(t.count(Kind::Write), 2);
}

#[test]
fn refusals_before_the_first_call_each_count() {
    let mut t = Tally::default();
    [&b"a"[..], b"b"].iter().for_each(|p| t.note(Kind::Exec, p));
    assert_eq!(t.count(Kind::Exec), 2);
}

#[test]
fn examples_are_the_first_few_cut_and_printable() {
    let mut t = Tally::default();
    let long = [b'a'; 200];
    for e in [&b"one"[..], b"tw\x1b[2Jo", &long, b"four"] {
        t.call_begins();
        t.note(Kind::Write, e);
    }
    let kept = t.examples(Kind::Write);
    assert_eq!(kept.len(), EXAMPLES);
    assert_eq!(kept[0], b"one");
    assert_eq!(kept[1], b"tw?[2Jo");
    assert_eq!(kept[2].len(), EXAMPLE_LEN);
    assert_eq!(t.count(Kind::Write), 4);
}
