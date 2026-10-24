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

//! A release cannot bring the verifiers that judge it: a tree at or inside
//! the release directory, given or found from where the command runs, is
//! refused before any check runs, and nothing it holds is run.

use super::fake::Fake;
use super::sealed_fixture::Sealed;
use crate::cli::CheckArgs;
use crate::exit;
use crate::plan::check_with;
use crate::tree::choose;

#[test]
fn a_tree_at_or_inside_the_release_is_refused() {
    let s = Sealed::dev();
    let rel = s.release.path();
    s.release.put("planted/nonos-verify/Cargo.toml", b"[package]\nname = \"planted\"\n");
    for tree in [rel.to_path_buf(), rel.join("planted")] {
        assert!(choose(Some(tree.clone()), rel).is_err(), "{tree:?} was taken");
        let (release, tree) = (rel.into(), Some(tree));
        let args = CheckArgs { release, tree, out: None, sign_key: None, json: true };
        let fake = Fake::default();
        let ran = fake.ran.clone();
        assert_eq!(check_with(&args, Box::new(fake)), exit::USAGE);
        assert!(ran.borrow().is_empty(), "something ran: {:?}", ran.borrow());
    }
    assert!(choose(Some(s.tree.path().into()), rel).is_ok_and(|t| t.is_some()));
}
