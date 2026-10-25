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

//! Spectre-BHB's part of the decision.

use super::answer::{Answer, Firmware};
use super::cores::bhb_loop;
use super::decide::quiet;
use super::ids::Ids;
use super::plan::{Bhb, Why};

/// ECBHB, or CSV2_3, means EL0's history cannot steer the kernel. Then the
/// surest first: CLRBHB is the architecture's own clear, the firmware knows
/// its core, and the loop is only as good as the count Arm published.
pub(super) fn bhb(ids: &Ids, fw: &Firmware) -> Bhb {
    if ids.ecbhb() >= 1 || ids.csv2() >= 3 {
        return Bhb::NotAffected(Why::Id);
    }
    if ids.clrbhb() >= 1 {
        return Bhb::Clrbhb;
    }
    match fw.wa3 {
        Answer::Call => return Bhb::Firmware,
        Answer::NotRequired => return Bhb::NotAffected(Why::Firmware),
        Answer::Unknown => {}
    }
    match bhb_loop(ids.implementer(), ids.part()) {
        Some(k) => Bhb::Loop(k),
        None if quiet(ids) => Bhb::NotAffected(Why::Core),
        None => Bhb::Vulnerable,
    }
}
