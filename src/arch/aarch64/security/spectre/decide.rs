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

//! The one decision: identification registers and firmware answers in, a
//! plan out. Pure, so `spectre_proofs` holds it to the table.

use super::answer::{Answer, Firmware};
use super::cores::{in_order, is_arm, meltdown as meltdown_core};
use super::history::bhb;
use super::ids::Ids;
use super::plan::{Meltdown, Plan, Ssbd, Why, V2};

pub(crate) fn decide(ids: &Ids, fw: &Firmware) -> Plan {
    Plan { v2: v2(ids, fw), bhb: bhb(ids, fw), ssbd: ssbd(ids, fw), meltdown: meltdown(ids) }
}

/// An in-order core: the answer when neither the ID registers nor the
/// firmware give one.
pub(super) fn quiet(ids: &Ids) -> bool {
    in_order(ids.implementer(), ids.part())
}

fn v2(ids: &Ids, fw: &Firmware) -> V2 {
    match (ids.csv2() >= 1, fw.wa1) {
        (true, _) => V2::NotAffected(Why::Id),
        (false, Answer::Call) => V2::Firmware,
        (false, Answer::NotRequired) => V2::NotAffected(Why::Firmware),
        (false, Answer::Unknown) if quiet(ids) => V2::NotAffected(Why::Core),
        (false, Answer::Unknown) => V2::Vulnerable,
    }
}

fn ssbd(ids: &Ids, fw: &Firmware) -> Ssbd {
    match (ids.ssbs() >= 1, fw.wa2) {
        (true, _) => Ssbd::Ssbs,
        (false, Answer::Call) => Ssbd::Firmware,
        (false, Answer::NotRequired) => Ssbd::NotAffected(Why::Firmware),
        (false, Answer::Unknown) if quiet(ids) => Ssbd::NotAffected(Why::Core),
        (false, Answer::Unknown) => Ssbd::Vulnerable,
    }
}

fn meltdown(ids: &Ids) -> Meltdown {
    let (maker, part) = (ids.implementer(), ids.part());
    if ids.csv3() >= 1 {
        Meltdown::NotAffected(Why::Id)
    } else if ids.e0pd() >= 1 {
        Meltdown::E0pd
    } else if meltdown_core(maker, part) {
        Meltdown::Exposed
    } else if is_arm(maker) {
        Meltdown::NotAffected(Why::Core)
    } else {
        Meltdown::Unknown
    }
}
