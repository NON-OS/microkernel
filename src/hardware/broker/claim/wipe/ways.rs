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

//! The boot line of how each function will be reset at its release (L78),
//! and each one nothing resets named: after its first holder it goes back
//! only to that holder's own driver service.

use super::caps::ResetCaps;
use super::method_rule::{method, Method};
use crate::drivers::pci::types::PciAddress;

pub(in crate::hardware::broker) fn say_reset_ways(found: &[(PciAddress, ResetCaps)]) {
    let ways = found.iter().map(|(_, c)| method(c.offers));
    let count = |m: Option<Method>| ways.clone().filter(|w| *w == m).count();
    crate::log::info!(
        "[BROKER] reset at release: flr={} af_flr={} d3hot={} none={}",
        count(Some(Method::Flr)),
        count(Some(Method::AfFlr)),
        count(Some(Method::PmCycle)),
        count(None)
    );
    for (address, _) in found.iter().filter(|(_, c)| method(c.offers).is_none()) {
        crate::log::info!(
            "[BROKER] {} has no reset: after its first holder only that driver service claims it",
            address
        );
    }
}
