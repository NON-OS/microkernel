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

//! What a claim on a described device comes to, from its `iommus` and the
//! unit's state. Pure, so the proofs run it over every case.

use crate::hardware::broker::described::DescribedIommus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Verdict {
    /// No `iommus`: nothing in the description puts it behind a unit.
    Unconfined,
    /// Attach every one of the device's streams, or none.
    Attach,
    Refuse(&'static [u8]),
}

/// Never `Unconfined` for a device the description puts behind a unit:
/// `Attach` only when that unit is the one in service (`in_service`) and
/// covers every one of its streams (`covered`).
pub(super) fn verdict(
    iommus: &DescribedIommus,
    in_service: impl Fn(u32) -> bool,
    covered: impl Fn(u32) -> bool,
) -> Verdict {
    match *iommus {
        DescribedIommus::None => Verdict::Unconfined,
        DescribedIommus::Unreadable(why) => Verdict::Refuse(why),
        DescribedIommus::Streams { unit, .. } if !in_service(unit) => {
            Verdict::Refuse(b"refused: its IOMMU is not one in service")
        }
        DescribedIommus::Streams { count: 0, .. } => {
            Verdict::Refuse(b"refused: iommus with no stream")
        }
        _ if iommus.sids().iter().any(|sid| !covered(*sid)) => {
            Verdict::Refuse(b"refused: a stream id its IOMMU does not cover")
        }
        _ => Verdict::Attach,
    }
}
