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

//! A program proved and bound once a launch. The run proves its program
//! before anything else runs, to know which role it belongs in
//! (run_net::early), and keeps what it found here for the start that
//! follows, which takes it instead of hashing the whole program again.

use core::cell::RefCell;

use alloc::vec::Vec;

use super::super::app::{bind, bind_built_in, AppIdentity};
use super::super::origin::Origin;

#[path = "kept_rule.rs"]
mod kept_rule;

type Kept = (Vec<u8>, usize, usize, AppIdentity);

struct One(RefCell<Option<Kept>>);

/*
 * SAFETY: the serve loop is the only thread in this capsule that proves a
 * program; guest threads run in their own processes and only trap into it.
 */
unsafe impl Sync for One {}

static KEPT: One = One(RefCell::new(None));

/// A program out of the store proves itself against the enrolled set, and
/// either kind is bound to the policy it runs under. The identity kept for
/// these very bytes at this path is taken instead, once.
pub(in crate::linux) fn prove(
    path: &[u8],
    bytes: &[u8],
    origin: &Origin,
) -> Result<AppIdentity, &'static str> {
    let mut kept = KEPT.0.borrow_mut();
    if let Some(k) = kept.take_if(|k| kept_rule::same((&k.0, k.1, k.2), path, bytes)) {
        return Ok(k.3);
    }
    drop(kept);
    match origin {
        Origin::BuiltIn => Ok(bind_built_in(bytes)),
        Origin::Store => super::super::attest::verify(path, bytes).and_then(|_| bind(path, bytes)),
    }
}

/// Keep `app`, proved for `bytes` at `path`, for the start that follows.
/// The bytes stay where they are until then: the launch holds them.
pub(in crate::linux) fn keep(path: &[u8], bytes: &[u8], app: AppIdentity) {
    *KEPT.0.borrow_mut() = Some((path.to_vec(), bytes.as_ptr() as usize, bytes.len(), app));
}
