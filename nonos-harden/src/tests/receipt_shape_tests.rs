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

//! The build receipt is read as untrusted: the shape scripts/build_receipt.py
//! writes is read, any other refused; a dirty sealing tree is stated, not hid.

use super::fake::Fake;
use super::receipt_tests::{edit, receipt_of, Change};
use super::sealed_fixture::Sealed;
use crate::check::Status;
use crate::release::{load, receipt_check};
use serde_json::{json, Value};

#[test]
fn a_receipt_in_any_other_shape_is_refused() {
    let changes: Vec<Change> = vec![
        Box::new(|v| *v = json!([])),
        Box::new(|v| v["schema"] = "nonos.build.receipt.v2".into()),
        Box::new(|v| v["source"]["commit"] = "HEAD".into()),
        Box::new(|v| v["source"]["commit"] = "a".repeat(41).into()),
        Box::new(|v| v["source"]["dirty"] = "no".into()),
        Box::new(|v| v["artifacts"] = json!({})),
        Box::new(|v| v["artifacts"]["/seal/out/nonos.img"]["sha256"] = "ABC".into()),
        Box::new(|v| v["roots"] = Value::Null),
        Box::new(|v| v["verification"] = json!({})),
    ];
    for (i, change) in changes.into_iter().enumerate() {
        let s = Sealed::dev();
        edit(&s, change);
        let mut ctx = s.ctx(Fake::default());
        load::check(&mut ctx);
        let c = receipt_check::check(&mut ctx);
        assert_eq!(c.status, Status::Fail, "change {i}: {}", c.reason);
        assert!(ctx.receipt.is_none(), "change {i}: a refused receipt names no commit");
    }
}

#[test]
fn without_a_receipt_the_release_fails_and_without_a_record_it_does_not_run() {
    let s = Sealed::dev();
    std::fs::remove_file(s.release.path().join("build-receipt.json")).unwrap();
    let mut ctx = s.ctx(Fake::default());
    load::check(&mut ctx);
    assert_eq!(receipt_check::check(&mut ctx).status, Status::Fail);
    let mut ctx = s.ctx(Fake::default());
    assert_eq!(receipt_check::check(&mut ctx).status, Status::NotRun);
}

#[test]
fn a_dirty_sealing_tree_is_stated_as_a_limit_and_not_hidden() {
    let s = Sealed::dev();
    s.receipt(true);
    let c = receipt_of(&s);
    assert_eq!(c.status, Status::Pass);
    assert!(c.limits.iter().any(|l| l.contains("changes beyond")), "{c:?}");
}
