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

//! The build receipt held against the release: its roots found embedded,
//! nothing it measured missing, and all it measured among the sealed bytes.

use super::fake::Fake;
use super::sealed_fixture::Sealed;
use crate::check::{Check, Status};
use crate::release::{load, receipt_check};
use serde_json::Value;
use std::fs;

pub fn receipt_of(s: &Sealed) -> Check {
    let mut ctx = s.ctx(Fake::default());
    load::check(&mut ctx);
    receipt_check::check(&mut ctx)
}

pub type Change = Box<dyn Fn(&mut Value)>;

pub fn edit(s: &Sealed, change: impl Fn(&mut Value)) {
    let at = s.release.path().join("build-receipt.json");
    let mut v: Value = serde_json::from_slice(&fs::read(&at).unwrap()).unwrap();
    change(&mut v);
    fs::write(at, v.to_string()).unwrap();
}

#[test]
fn the_receipt_the_seal_writes_agrees_and_names_its_commit() {
    let s = Sealed::dev();
    let c = receipt_of(&s);
    assert_eq!((c.status, c.limits.len()), (Status::Pass, 0), "{c:?}");
    assert!(c.reason.contains(&s.tree.commit));
}

#[test]
fn a_receipt_that_disagrees_with_the_release_fails() {
    let other = "b".repeat(64);
    let changes: Vec<(Change, &str)> = vec![
        (Box::new(|v| v["verification"]["root_embedding"] = "fail".into()), "not embedded"),
        (
            Box::new(|v| v["roots"]["capsule_policy"]["embedded_in_kernel"] = false.into()),
            "policy root",
        ),
        (Box::new(|v| v["roots"]["kernel_attest"]["ok"] = false.into()), "attestation root"),
        (
            Box::new(move |v| {
                v["artifacts"]["/seal/out/nonos.img"]["sha256"] = other.clone().into()
            }),
            "nonos.img",
        ),
        (Box::new(|v| v["artifacts"]["/seal/out/nonos.img"] = Value::Null), "missing when"),
    ];
    for (change, named) in changes {
        let s = Sealed::dev();
        edit(&s, change);
        let c = receipt_of(&s);
        assert_eq!(c.status, Status::Fail, "{named}");
        assert!(c.reason.contains(named), "{named} not in {}", c.reason);
    }
}
