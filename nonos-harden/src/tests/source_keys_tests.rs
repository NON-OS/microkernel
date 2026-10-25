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

//! Only where the seal stages may the tree differ, and of that the kernel's
//! public keys are pinned: a release whose tree holds other kernel keys than
//! its commit fails, and a development image's own pass only as a limit.

use super::fake::Fake;
use super::sealed_fixture::Sealed;
use super::source_tests::{fails, passes};
use crate::check::{Check, Status};
use crate::release::kernel_keys::ED_PUB;
use crate::release::{kind, load, receipt_check};
use crate::tree::source;

fn with_other_kernel_key(s: &Sealed) -> Check {
    std::fs::write(s.tree.path().join(ED_PUB), [9u8; 32]).unwrap();
    let mut ctx = s.ctx(Fake::default());
    load::check(&mut ctx);
    kind::check(&mut ctx);
    receipt_check::check(&mut ctx);
    source::check(&mut ctx)
}

#[test]
fn a_release_tree_with_other_kernel_keys_than_its_commit_fails() {
    let c = with_other_kernel_key(&Sealed::release());
    assert!(c.status == Status::Fail && c.reason.contains("public keys differ"), "{c:?}");
}

#[test]
fn a_development_trees_own_kernel_keys_are_a_stated_limit() {
    let c = with_other_kernel_key(&Sealed::dev());
    assert_eq!(c.status, Status::Pass, "{c:?}");
    assert!(c.limits.iter().any(|l| l.contains("development tree's own")), "{c:?}");
}

#[test]
fn only_where_the_seal_stages_may_differ_and_outputs_and_key_stores_are_not_walked() {
    let s = Sealed::dev();
    s.tree.put("nonos-data/generated_store/x.rs", b"not staged by the seal");
    fails(&s, "nonos-data/generated_store/x.rs");
    let s = Sealed::dev();
    s.tree.put("userland/x/target/debug/out", b"a build output");
    s.tree.put("tools/__pycache__/m.cpython-313.pyc", b"a bytecode cache");
    s.tree.put(".keys/placeholder", b"stands in for a key store's file");
    s.tree.put("nonos-bootloader/keys/placeholder", b"stands in for a key store's file");
    passes(&s, "exactly commit");
}
