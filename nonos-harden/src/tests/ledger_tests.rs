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

//! `tree.ledger`: the trust set matches its ledger file for file; a changed
//! byte, a file missing from the set or one added beside the ledger fails it,
//! and the ledger itself is read strictly.

use super::fake::Fake;
use super::sealed_fixture::Sealed;
use crate::check::{Check, Status};
use crate::tree::ledger::{self, TRUST};
use crate::tree::manifest::parse;

fn ledger_of(s: &Sealed) -> Check {
    ledger::check(&mut s.ctx(Fake::default()))
}

fn fails(s: &Sealed, why: &str) {
    let c = ledger_of(s);
    assert!(c.status == Status::Fail && c.reason.contains(why), "{why:?}: {c:?}");
}

#[test]
fn the_trust_set_matches_its_ledger() {
    let c = ledger_of(&Sealed::dev());
    assert_eq!(c.status, Status::Pass, "{c:?}");
}

#[test]
fn a_changed_missing_or_added_file_fails_and_is_named() {
    let s = Sealed::dev();
    s.tree.put(&format!("{TRUST}/capsules/about.manifest.bin"), b"a capsule manifest, changed");
    fails(&s, "capsules/about.manifest.bin: sha256");
    let s = Sealed::dev();
    std::fs::remove_file(s.tree.path().join(TRUST).join("policy/kernel_attest_root.bin")).unwrap();
    fails(&s, "policy/kernel_attest_root.bin is missing");
    let s = Sealed::dev();
    s.tree.put(&format!("{TRUST}/keys/added.pub"), b"a key nobody stamped");
    fails(&s, "keys/added.pub is in the trust set but not in its ledger");
}

#[test]
fn the_ledger_is_read_strictly() {
    let h = "c".repeat(64);
    assert_eq!(parse(format!("{h}  keys/a.pub\n{h} *keys/b.pub\n").as_bytes()).unwrap().len(), 2);
    for bad in [
        String::new(),
        format!("{h} keys/a.pub\n"),
        format!("{}  keys/a.pub\n", "C".repeat(64)),
        format!("{h}  ../a.pub\n"),
        format!("{h}  keys/a.pub\r\n"),
        format!("{h}  keys/a.pub\n{h}  keys/a.pub\n"),
        format!("{h}  keys/a.pub\n\n"),
    ] {
        assert!(parse(bad.as_bytes()).is_err(), "{bad:?} was read");
    }
    assert!(parse(b"\xff\xfe").is_err());
}
