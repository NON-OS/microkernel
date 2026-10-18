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

//! A plan written as each encoding's bytes.

use super::plan::Plan;
use super::rng::Rng;
use crate::build::dtd;
use crate::build_ext::type7;

/// As an 18 byte descriptor, with any flags: no flag moves the bar.
pub fn as_dtd(p: &Plan, r: &mut Rng) -> [u8; 18] {
    let mm = (r.range(0, 4095), r.range(0, 4095));
    dtd(p.khz, p.t, mm, r.byte())
}

/// As a 20 byte DisplayID record counting its clock in `unit_khz`.
pub fn as_record(p: &Plan, unit_khz: u32, r: &mut Rng) -> [u8; 20] {
    type7(p.khz / unit_khz, p.t, r.byte())
}
