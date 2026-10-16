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

//! A claim refused for want of a domain, said once per claim.

use super::decide::Verdict;
use super::table::say;

/// No unit confines the device, or the one in service would not take it, so
/// the claim is refused rather than granted with a device that reaches all
/// of memory (decide.rs).
pub(in crate::hardware::broker) struct Refused;

/// A PCI device with no domain of its own: refused, but under the opt-in.
pub(super) fn without_domain(v: Verdict) -> Result<(), Refused> {
    if v == Verdict::Unconfined {
        say(b"unconfined under the development opt-in, reaches all memory");
        return Ok(());
    }
    say(b"refused: no remapping unit confines it");
    Err(Refused)
}
