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

//! A processor's idle hierarchy (ACPI 6.5, 8.4.2 and 8.4.4): its own _LPI,
//! then that of each processor container (ACPI0010) it sits in, nearest
//! first. A container is a device the processor is defined inside; any
//! scope between that is no container is passed over.

use super::container_uid::container_uid;
use super::lpi::parse_lpi;
use super::lpi_types::Lpi;
use crate::load::Aml;
use crate::namespace::{Id, ROOT};
use crate::platform::{child, Eval};
use alloc::vec::Vec;

pub use super::container_uid::CONTAINER_HID;
/// Levels beyond the processor's own; deeper hierarchies are cut here.
pub const MAX_CONTAINERS: usize = 3;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LpiLevel {
    /// The container's namespace id and _UID, which processors that share
    /// the level share; None for the processor's own level.
    pub container: Option<(Id, u64)>,
    pub lpi: Lpi,
}

fn lpi(aml: &mut Aml, eval: &mut Eval<'_>, id: Id) -> Option<Lpi> {
    parse_lpi(&child(aml, eval, id, *b"_LPI", Vec::new())?.ok()?)
}

/// Empty when the processor has no _LPI of its own (it idles by _CST).
/// The walk stops at the first container whose _UID or _LPI is missing or
/// malformed: the level below names its states, so none above it is
/// usable either.
pub fn idle_levels(aml: &mut Aml, eval: &mut Eval<'_>, cpu: Id) -> Vec<LpiLevel> {
    let Some(own) = lpi(aml, eval, cpu) else { return Vec::new() };
    let mut out = alloc::vec![LpiLevel { container: None, lpi: own }];
    let mut at = cpu;
    while out.len() <= MAX_CONTAINERS {
        let Some(up) = aml.ns.get(at).map(|e| e.parent).filter(|p| *p != at) else { break };
        at = up;
        if at == ROOT {
            break;
        }
        match container_uid(aml, eval, at) {
            Some(None) => continue,
            Some(Some(uid)) => match lpi(aml, eval, at) {
                Some(l) => out.push(LpiLevel { container: Some((at, uid)), lpi: l }),
                None => break,
            },
            None => break,
        }
    }
    out
}
