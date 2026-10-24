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

//! A gap is a limit only if the tree declares it for the module stating it:
//! a passing report stating any other gap fails, and so does every verifier
//! when the list is missing or not in its format, even one stating none.

use super::fake::{report, Fake};
use super::fake_report::{gap_everywhere, other_gap};
use super::judge_tests::trust_chain;
use super::sealed_fixture::Sealed;
use crate::check::Status;
use crate::exit;
use crate::tree::declared::{parse, FILE};
use crate::tree::verify;
use std::fs;

#[test]
fn a_gap_the_tree_does_not_declare_is_no_go() {
    let c = trust_chain(Fake { report: other_gap, ..Fake::default() });
    assert!(c.status == Status::Fail && c.reason.contains("a new gap"), "{c:?}");
    let (code, v) = Sealed::dev().verdict(Fake { report: other_gap, ..Fake::default() });
    assert_eq!((code, v["overall"].as_str()), (exit::NO_GO, Some("no-go")), "{v:#}");
}

#[test]
fn a_gap_declared_for_one_module_is_undeclared_for_another() {
    let c =
        verify::hygiene(&mut Sealed::dev().ctx(Fake { report: gap_everywhere, ..Fake::default() }));
    assert!(c.status == Status::Fail && c.reason.contains("field asserts"), "{c:?}");
}

#[test]
fn without_a_declared_list_in_its_format_no_verifier_passes() {
    let header = b"# format: nonos-harden-declared-limits/1\n";
    for list in [
        None,
        Some(&b"trust-chain: field asserts\n"[..]),
        Some(&[&header[..], b"no colon\n"].concat()[..]),
    ] {
        let s = Sealed::dev();
        let at = s.tree.path().join(FILE);
        match list {
            None => fs::remove_file(&at).unwrap(),
            Some(b) => fs::write(&at, b).unwrap(),
        }
        // A report stating no gap at all: the list is needed even then.
        let fake = Fake { report: |_| Some(report(&[("a", "pass")], &[])), ..Fake::default() };
        let c = verify::trust_chain(&mut s.ctx(fake));
        assert!(c.status == Status::Fail && c.reason.contains(FILE), "{list:?}: {c:?}");
    }
    assert!(parse(b"# format: nonos-harden-declared-limits/2\n").is_err());
    assert_eq!(parse(header).unwrap(), vec![]);
}
