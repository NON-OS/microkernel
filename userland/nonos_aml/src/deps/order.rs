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

//! Devices put in an order where each comes after the devices it depends
//! on among those being placed. A dependency outside that set must be
//! present (`present`); one that is not, or a dangling one, leaves the
//! device Unmet. A device depending on an Unmet one is Unmet too. Devices
//! in a cycle, or waiting on one, come last as Cycle. Every device appears
//! exactly once. Pure.

use super::read::Dep;
use crate::namespace::Id;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Ready,
    Unmet,
    Cycle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Placed {
    pub id: Id,
    pub verdict: Verdict,
}

pub fn order(devices: &[(Id, Vec<Dep>)], present: &dyn Fn(Id) -> bool) -> Vec<Placed> {
    let ours = |id: Id| devices.iter().any(|(d, _)| *d == id);
    let mut out: Vec<Placed> = Vec::with_capacity(devices.len());
    // each pass places at least one device, or the rest are cycles
    for _ in 0..devices.len() {
        let before = out.len();
        for (id, deps) in devices {
            if out.iter().any(|p| p.id == *id) {
                continue;
            }
            let placed = |d: Id| out.iter().find(|p| p.id == d).map(|p| p.verdict);
            let waits =
                deps.iter().any(|d| matches!(d, Dep::On(x) if ours(*x) && placed(*x).is_none()));
            if waits {
                continue;
            }
            let unmet = deps.iter().any(|d| match d {
                Dep::Dangling => true,
                Dep::On(x) if ours(*x) => placed(*x) != Some(Verdict::Ready),
                Dep::On(x) => !present(*x),
            });
            let verdict = if unmet { Verdict::Unmet } else { Verdict::Ready };
            out.push(Placed { id: *id, verdict });
        }
        if out.len() == before {
            break;
        }
    }
    for (id, _) in devices {
        if !out.iter().any(|p| p.id == *id) {
            out.push(Placed { id: *id, verdict: Verdict::Cycle });
        }
    }
    out
}
