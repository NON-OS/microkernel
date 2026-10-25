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

//! What `inspect` tells the operator about PCR 14: each capsule, how it was
//! admitted, and whether a release would take them.

use crate::build::tcg_capsule::{path_only_tag, PATH_ONLY};
use crate::build::Spec;

fn lines(s: Spec) -> Vec<String> {
    let b = s.build();
    let out = nsh_enrol::inspect(&b.doc, None).expect("inspected");
    out.lines()
        .filter(|l| l.starts_with("capsule") || l.starts_with("refused"))
        .map(Into::into)
        .collect()
}

#[test]
fn each_capsule_is_shown_with_how_it_was_admitted() {
    let got = lines(Spec::default());
    assert_eq!(got.len(), 4, "{got:?}");
    let first = format!("capsule    vendor cert {} verifier {}", "01".repeat(32), "c1".repeat(32));
    assert_eq!(got[0], first);
    assert_eq!(got[3], "capsules   3, pcr 14 ok true");
}

#[test]
fn a_path_only_capsule_is_shown_and_no_release_takes_it() {
    let mut s = Spec::default();
    s.log.capsule([4; 32], path_only_tag(), PATH_ONLY);
    let got = lines(s);
    assert!(got[3].starts_with("capsule    path-only cert 0404"), "{got:?}");
    assert_eq!(got[4], "capsules   4, pcr 14 ok false");
    assert_eq!(got[5], "refused    capsule 3, path-only");
}
