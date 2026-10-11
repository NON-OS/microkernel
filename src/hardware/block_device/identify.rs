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

//! Whether one backend's disk is the NONOS disk, and where its parts lie.

use super::found::Found;
use super::gpt::{self, holds, FIXED};
use super::seen::Seen;
use super::{backend::Backend, capacity::capacity_on, fit::sectors_fit, read::head};

/// The store's and the plan's magics, as vfs and `blockfs_volume` write
/// them at their first sectors.
const STORE_MAGIC: &[u8; 8] = b"NONOSTR1";
const PLAN_MAGIC: &[u8; 8] = b"NONOSDP1";

/// What `backend`'s disk is, and what was seen on it, which select.rs says
/// on the log. A table naming NONOS's partitions places the store and plan;
/// a disk without one (a stick, a QEMU data disk) has them at `FIXED`.
pub(super) fn identify(backend: Backend, boot: Option<&[u8; 16]>) -> (Found, Seen) {
    let sectors = match capacity_on(backend) {
        Ok(s) => s,
        Err(e) => return (Found::of(e), Seen::NoSize(e)),
    };
    if sectors <= FIXED.store.first {
        return (Found::Absent, Seen::TooSmall(sectors));
    }
    if !sectors_fit(backend) {
        return (Found::Absent, Seen::NotFiveTwelve(sectors));
    }
    let table = match gpt::table(backend, sectors) {
        Ok(t) => t,
        Err(e) => return (Found::of(e), Seen::NoTable(sectors, e)),
    };
    let named = table.as_deref().and_then(gpt::places);
    let places = named.unwrap_or(FIXED);
    let boot = matches!((table.as_deref(), boot), (Some(t), Some(g)) if holds(t, g));
    let kept = || Found::Layout { places, boot };
    let store = head(backend, places.store.first);
    let gpt = named.is_some();
    let seen = |plan| Seen::At { sectors, lba: places.store.first, got: store, plan, gpt, boot };
    match store {
        Ok(h) if &h == STORE_MAGIC => return (kept(), seen(None)),
        Err(e) => return (Found::of(e), seen(None)),
        Ok(_) if sectors <= places.plan.first => return (Found::Absent, seen(None)),
        Ok(_) => {}
    }
    let plan = head(backend, places.plan.first);
    let found = match plan {
        Ok(h) if &h == PLAN_MAGIC => kept(),
        Ok(_) => Found::Absent,
        Err(e) => Found::of(e),
    };
    (found, seen(Some(plan)))
}
