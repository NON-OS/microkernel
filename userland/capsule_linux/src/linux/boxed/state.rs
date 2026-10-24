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

//! The box this run is held in, armed once from the command line, and what
//! it refused. Only the serve loop reaches it, as with the socket table.

use alloc::vec::Vec;
use core::cell::RefCell;

use super::hash::box_hash;
use super::kind::Kind;
use super::preset::BoxPolicy;
use super::tally::Tally;
use crate::linux::app::AppIdentity;

pub(super) struct Armed {
    pub(super) policy: BoxPolicy,
    pub(super) hash: [u8; 32],
    pub(super) tally: Tally,
}

pub(super) struct One(pub(super) RefCell<Option<Armed>>);

/*
 * SAFETY: the serve loop is the only thread in this capsule that reaches the
 * box; guest threads run in their own processes and only trap into it.
 */
unsafe impl Sync for One {}

pub(super) static BOX: One = One(RefCell::new(None));

pub fn arm(policy: BoxPolicy) {
    let hash = box_hash(&policy);
    *BOX.0.borrow_mut() = Some(Armed { policy, hash, tally: Tally::default() });
}

/// The program's identity inside the box: never more than both allow.
pub fn narrow(app: AppIdentity) -> AppIdentity {
    match BOX.0.borrow().as_ref() {
        None => app,
        // The list lives as long as the run: the identity holds it by 'static.
        Some(a) => app.boxed(a.policy.mask, Vec::leak(a.policy.dests.clone())),
    }
}

pub fn may_write(path: &[u8]) -> bool {
    BOX.0.borrow().as_ref().is_none_or(|a| super::folders::may_write(&a.policy.folders, path))
}

pub fn call_begins() {
    if let Some(a) = BOX.0.borrow_mut().as_mut() {
        a.tally.call_begins();
    }
}

pub fn note(kind: Kind, example: &[u8]) {
    if let Some(a) = BOX.0.borrow_mut().as_mut() {
        a.tally.note(kind, example);
    }
}
