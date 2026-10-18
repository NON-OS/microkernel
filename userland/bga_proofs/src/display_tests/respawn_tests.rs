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

//! A compositor that died and was spawned again under a new pid. The
//! primary was its first asker's, so the new one was locked out for good;
//! it is the pid the name is registered to now, asked at each get_primary,
//! so the new one takes it and the old pid has nothing, not even a pin.

use nonos_libc::hold;

use super::frames::{ask, flush_body};
use super::model::{served, COMPOSITOR, OTHER};
use crate::serve::{COMPOSITOR_SERVICE, E_BUSY, OP_FLUSH, OP_GET_PRIMARY_SURFACE};

/// The compositor as init spawns it again once the first one died.
pub const RESPAWNED: u32 = 0x45;

#[test]
fn a_respawned_compositor_takes_the_primary_once_the_old_one_is_gone() {
    let mut s = served(64, 48);
    let flush = flush_body(0, 0, 1, 1);
    assert_eq!(ask(&mut s.d, COMPOSITOR, OP_GET_PRIMARY_SURFACE, &[]).0, 0);
    let early = ask(&mut s.d, RESPAWNED, OP_GET_PRIMARY_SURFACE, &[]).0;
    assert_eq!(early, E_BUSY, "taken while the compositor still runs");
    hold(COMPOSITOR_SERVICE, None);
    hold(COMPOSITOR_SERVICE, Some(RESPAWNED));
    let stale = ask(&mut s.d, COMPOSITOR, OP_GET_PRIMARY_SURFACE, &[]).0;
    assert_eq!(stale, E_BUSY, "the old compositor's pid kept the primary on its pin");
    let taken = ask(&mut s.d, RESPAWNED, OP_GET_PRIMARY_SURFACE, &[]).0;
    assert_eq!(taken, 0, "the respawned compositor was locked out");
    assert_eq!(ask(&mut s.d, RESPAWNED, OP_FLUSH, &flush).0, 0);
    assert_eq!(ask(&mut s.d, COMPOSITOR, OP_FLUSH, &flush).0, E_BUSY, "the old one kept it");
    assert_eq!(ask(&mut s.d, OTHER, OP_GET_PRIMARY_SURFACE, &[]).0, E_BUSY);
}
