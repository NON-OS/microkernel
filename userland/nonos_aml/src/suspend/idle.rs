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

//! Suspend-to-idle's notifications around idle entry, in the order
//! Microsoft documents for Modern Standby: display off (3), Modern Standby
//! entry (7, Microsoft's set only), sleep entry (5); and on the way out
//! sleep exit (6), Modern Standby exit (8), display on (4). Each phase
//! calls every offered set, Microsoft's first, and only the functions the
//! set's function 0 lists. Numbers recalled, unverified (`lps0.rs`).

use super::lps0::{dsm, Lps0};
use super::vendor::Vendor;
use crate::load::Aml;
use crate::platform::Eval;
use alloc::vec::Vec;

pub const DISPLAY_OFF: u64 = 3;
pub const DISPLAY_ON: u64 = 4;
pub const ENTRY: u64 = 5;
pub const EXIT: u64 = 6;
pub const MS_ENTRY: u64 = 7;
pub const MS_EXIT: u64 = 8;

/// A call made: the set, the function, and whether it answered.
pub type Call = (Vendor, u64, bool);

fn phases(aml: &mut Aml, eval: &mut Eval<'_>, l: &Lps0, order: [u64; 3]) -> Vec<Call> {
    let mut calls = Vec::new();
    for f in order {
        for (v, mask) in [(Vendor::Microsoft, l.microsoft), (Vendor::Intel, l.intel)] {
            let ms_only = matches!(f, MS_ENTRY | MS_EXIT) && v != Vendor::Microsoft;
            if ms_only || mask & (1 << f) == 0 {
                continue;
            }
            calls.push((v, f, dsm(aml, eval, l.dev, v, f).is_some()));
        }
    }
    calls
}

pub fn idle_enter(aml: &mut Aml, eval: &mut Eval<'_>, l: &Lps0) -> Vec<Call> {
    phases(aml, eval, l, [DISPLAY_OFF, MS_ENTRY, ENTRY])
}

pub fn idle_exit(aml: &mut Aml, eval: &mut Eval<'_>, l: &Lps0) -> Vec<Call> {
    phases(aml, eval, l, [EXIT, MS_EXIT, DISPLAY_ON])
}
