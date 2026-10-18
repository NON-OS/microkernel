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

//! The posture line's words for each part of a plan.

use super::plan::{Bhb, Meltdown, Ssbd, Why, V2};

fn unaffected(why: Why) -> &'static [u8] {
    match why {
        Why::Id => b"unaffected (ID registers)",
        Why::Firmware => b"unaffected (firmware)",
        Why::Core => b"unaffected (core)",
    }
}

pub(super) fn v2(v2: V2) -> &'static [u8] {
    match v2 {
        V2::NotAffected(why) => unaffected(why),
        V2::Firmware => b"ARCH_WORKAROUND_1 on EL0 entry",
        V2::Vulnerable => b"VULNERABLE",
    }
}

/// The loop's count follows on the line, so `Loop` names only the method.
pub(super) fn bhb(bhb: Bhb) -> &'static [u8] {
    match bhb {
        Bhb::NotAffected(why) => unaffected(why),
        Bhb::Clrbhb => b"CLRBHB on EL0 entry",
        Bhb::Firmware => b"ARCH_WORKAROUND_3 on EL0 entry",
        Bhb::Loop(_) => b"history loop on EL0 entry, k=",
        Bhb::Vulnerable => b"VULNERABLE",
    }
}

pub(super) fn ssbd(ssbd: Ssbd) -> &'static [u8] {
    match ssbd {
        Ssbd::NotAffected(why) => unaffected(why),
        Ssbd::Ssbs => b"SSBS",
        Ssbd::Firmware => b"ARCH_WORKAROUND_2",
        Ssbd::Vulnerable => b"VULNERABLE",
    }
}

pub(super) fn meltdown(meltdown: Meltdown) -> &'static [u8] {
    match meltdown {
        Meltdown::NotAffected(why) => unaffected(why),
        Meltdown::E0pd => b"E0PD",
        Meltdown::Exposed => b"EXPOSED, kernel mapped while EL0 runs",
        Meltdown::Unknown => b"unknown core",
    }
}
