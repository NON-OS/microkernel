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
use super::preset::BoxPolicy;
use super::tally::Tally;
use crate::linux::app::AppIdentity;

pub(super) struct Armed {
    pub(super) policy: BoxPolicy,
    pub(super) hash: [u8; 32],
    pub(super) tally: Tally,
    /// The key the starting terminal named for the report (frames.rs).
    pub(super) key: Option<[u8; super::frames::KEY_LEN]>,
    /// The services the program was given in the box, once it started.
    pub(super) granted: Option<u64>,
    pub(super) connected: super::connected::Connected,
    pub(super) learned: super::learned::Learned,
}

pub(super) struct One(pub(super) RefCell<Option<Armed>>);

/*
 * SAFETY: the serve loop is the only thread in this capsule that reaches the
 * box; guest threads run in their own processes and only trap into it.
 */
unsafe impl Sync for One {}

pub(super) static BOX: One = One(RefCell::new(None));

pub fn arm(policy: BoxPolicy, key: Option<[u8; super::frames::KEY_LEN]>) {
    let hash = box_hash(&policy);
    let tally = Tally::default();
    let (connected, learned) = Default::default();
    *BOX.0.borrow_mut() =
        Some(Armed { policy, hash, tally, key, granted: None, connected, learned });
}

/// The program's identity inside the box: never more than both allow.
pub fn narrow(app: AppIdentity) -> AppIdentity {
    let mut armed = BOX.0.borrow_mut();
    let Some(a) = armed.as_mut() else { return app };
    // The list lives as long as the run: the identity holds it by 'static.
    let boxed = app.boxed(a.policy.mask, Vec::leak(a.policy.dests.clone()));
    a.granted = Some(boxed.mask);
    boxed
}

/// A program the run execs, as the box sees it before the exec narrows it to
/// the program that exec'd it: widened as the first one was (identity_box.rs).
pub fn exec_program(app: AppIdentity) -> AppIdentity {
    BOX.0.borrow().as_ref().map_or(app, |a| app.widened(&a.policy.dests))
}
