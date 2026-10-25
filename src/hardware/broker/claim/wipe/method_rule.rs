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

//! Which reset a function is given (L78): a Function Level Reset where its
//! PCIe capability offers one, else the Advanced Features one, else a trip
//! to D3hot and back where the function does not keep its state through it
//! (No_Soft_Reset clear); after an FLR its marker survived, the trip once.

use super::bits_rule::{AF_FLR_CAP, FLR_CAP, NO_SOFT_RESET};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Method {
    Flr,
    AfFlr,
    PmCycle,
}

/// What the function offers, read once at enumeration.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) struct Offers {
    pub(crate) flr: bool,
    pub(crate) af_flr: bool,
    pub(crate) soft_reset_d3: bool,
}

/// From Device Capabilities, the Advanced Features capabilities byte and
/// PMCSR, each none when the function has no such capability.
pub(crate) const fn offers(devcap: Option<u32>, af_caps: Option<u8>, pmcsr: Option<u16>) -> Offers {
    Offers {
        flr: matches!(devcap, Some(c) if c & FLR_CAP != 0),
        af_flr: matches!(af_caps, Some(c) if c & AF_FLR_CAP != 0),
        soft_reset_d3: matches!(pmcsr, Some(p) if p & NO_SOFT_RESET == 0),
    }
}

pub(crate) const fn method(o: Offers) -> Option<Method> {
    if o.flr {
        Some(Method::Flr)
    } else if o.af_flr {
        Some(Method::AfFlr)
    } else if o.soft_reset_d3 {
        Some(Method::PmCycle)
    } else {
        None
    }
}

/// The one other way tried after `tried` left its marker set.
pub(crate) const fn fallback(tried: Method, o: Offers) -> Option<Method> {
    match tried {
        Method::Flr | Method::AfFlr if o.soft_reset_d3 => Some(Method::PmCycle),
        _ => None,
    }
}

/// Whether the function may reach memory past Bus Master off: VMMDev, which
/// reads guest memory past any gate, and an Intel display function, whose
/// planes may fetch with Bus Master off (recalled, unverified).
pub(crate) const fn binds_past_bus_master(untranslated: bool, vendor: u16, class: u8) -> bool {
    untranslated || (vendor == 0x8086 && class == 0x03)
}
