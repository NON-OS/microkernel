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

//! A learn run can never be taken for an enforced one: its own word only, its
//! own hash, LEARN on its report, `enforced=no` on its `[BOX]` line, and the
//! file it learned; every other box reports as it did.

use crate::boxed::hash::box_hash;
use crate::boxed::learn_report::{enforced, learned, mode, MODE};
use crate::boxed::learned::Learned;
use crate::boxed::line::box_line;
use crate::boxed::preset::{parse, Preset};
use crate::boxed::report::{report, End};
use crate::boxed::tally::Tally;
use crate::boxed::terminal_run::{for_terminal, NO_LIST};

const RAN: End<'static> = End { exit: 0, not_started: None, reached: &[] };

#[test]
fn only_the_word_learn_is_a_learn_run_and_it_hashes_as_its_own() {
    let (learn, offline) = (parse(b"learn").unwrap(), parse(b"offline").unwrap());
    assert_eq!(learn.preset, Preset::Learn);
    assert_eq!((learn.mask, &learn.folders), (offline.mask, &offline.folders), "held as offline");
    assert_ne!(box_hash(&learn), box_hash(&offline));
    for word in [&b"learn:"[..], b"LEARN", b"learn ", b"learnx", b"allow:learn", b"strictlearn"] {
        assert!(parse(word).map_or(true, |p| p.preset != Preset::Learn), "{word:?}");
    }
    assert_eq!(for_terminal(b"learn", true), Err(NO_LIST), "never in the network role");
}

#[test]
fn a_learn_report_says_learn_and_not_enforced() {
    let p = parse(b"learn").unwrap();
    let text = report(&p, &box_hash(&p), &Tally::default(), None, &RAN);
    let policy = text.lines().position(|l| l.starts_with("  policy   learn ")).unwrap();
    assert_eq!(text.lines().nth(policy + 1), Some(format!("  {MODE}").as_str()));
    assert!(MODE.contains("LEARN, not enforced"));
    let line = box_line(&p, &box_hash(&p), &Tally::default(), &RAN);
    assert!(line.starts_with("[BOX] v=1 preset=learn enforced=no policy="), "{line}");
    let file = learned(&p, &Learned::default(), &Tally::default()).unwrap();
    assert!(file.starts_with("  learned  the tightest nonos-box 1 file for what it did:\n"));
    assert!(file.contains("\n    nonos-box 1\n") && file.contains("\n    base strict\n"));
}

#[test]
fn no_other_box_says_learn() {
    for word in [&b"strict"[..], b"offline", b"allow:/tmp/o", b"allow:/tmp/o,net:a.b:1"] {
        let p = parse(word).unwrap();
        let text = report(&p, &box_hash(&p), &Tally::default(), None, &RAN);
        let line = box_line(&p, &box_hash(&p), &Tally::default(), &RAN);
        assert!(!text.contains("LEARN") && !line.contains("enforced="), "{text}{line}");
        assert_eq!((mode(&p), enforced(&p)), (None, ""));
        assert!(learned(&p, &Learned::default(), &Tally::default()).is_none());
    }
}
