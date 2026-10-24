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

//! What the first boot-gate run over a real development seal found, held:
//! the kernel's Ed25519 key is the raw 32 bytes the seal writes (the tree's
//! own committed keys read as they are), the kernel key in that form cannot
//! sign a verdict, and a relative --release or --tree reaches every verifier
//! as an absolute path, whatever directory it runs from.

use super::fake::Fake;
use super::sealed_fixture::Sealed;
use super::sign_refuse_tests::refused;
use super::signing::operator;
use crate::cli::CheckArgs;
use crate::exit;
use crate::plan::check_with;
use crate::release::kernel_keys::KernelKeys;
use std::path::{Path, PathBuf};

#[test]
fn the_kernel_keys_this_tree_commits_read_in_the_seals_formats() {
    let tree = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    assert!(KernelKeys::load(tree).is_ok(), "{:?}", KernelKeys::load(tree).err());
}

#[test]
fn the_raw_kernel_key_cannot_sign_a_verdict() {
    let (s, dir) = (Sealed::dev(), tempfile::tempdir().unwrap());
    // The fixture's kernel key is seed 7, published raw as the seal does.
    let (path, _) = operator(dir.path(), 7);
    refused(&s, &path);
}

// `to` as a path relative to this process's working directory.
fn relative(to: &Path) -> PathBuf {
    let here = std::env::current_dir().unwrap();
    let up = here.components().count() - 1;
    let mut p: PathBuf = std::iter::repeat_n("..", up).collect();
    p.push(to.strip_prefix("/").unwrap());
    p
}

#[test]
fn a_relative_release_and_tree_reach_every_verifier_as_absolute_paths() {
    let s = Sealed::dev();
    let (release, tree) = (relative(s.release.path()), Some(relative(s.tree.path())));
    assert!(release.is_relative());
    let args = CheckArgs { release, tree, out: None, sign_key: None, json: true };
    let fake = Fake::default();
    let ran = fake.ran.clone();
    // The stand-in refuses any command given a relative path or directory.
    assert_eq!(check_with(&args, Box::new(fake)), exit::DEV_GO);
    assert_eq!(ran.borrow().len(), 6, "every verifier ran: {:?}", ran.borrow());
}
