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

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::find::iommus::{iommus, Iommus};
use crate::arch::fdt::node::NodeRef;
use crate::arch::fdt::parser::Fdt;

use super::entry::Owner;
use super::owner::Found;

/// A platform device keyed by its `iommus` stream ids.
pub(super) fn streams(fdt: &Fdt, device: &NodeRef) -> Result<Found, FdtError> {
    Ok(match iommus(fdt, device)? {
        Iommus::None => Err(b"a device with no iommus to key it by"),
        Iommus::Streams { sids, count, .. } => Ok(Owner::Streams { sids, count }),
        Iommus::Unreadable(why) => Err(why),
    })
}
