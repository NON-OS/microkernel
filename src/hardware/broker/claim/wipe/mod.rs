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

//! A device wiped between holders (L78). A PCI function's reset runs when its
//! last claim's entry goes, stepped one act at a time, every wait a deadline;
//! a claim while it runs is told Resetting before any entry exists; a
//! function no reset can be trusted on goes back only to its last holder's
//! own driver service. The pure rules are mounted by the proof crate.

mod act;
mod admit_rule;
mod ask;
mod bits_rule;
mod caps;
mod check;
mod due_rule;
mod end_rule;
mod finish;
mod held;
mod holder;
mod holder_rule;
mod marker_rule;
mod method_rule;
mod phase_rule;
mod reset;
mod start_rule;
mod step;
mod table;
mod ways;

pub(in crate::hardware::broker) use admit_rule::Admit;
pub(in crate::hardware::broker) use ask::{admits, owe, rearm};
pub(in crate::hardware::broker) use caps::{reset_caps, ResetCaps};
pub(in crate::hardware::broker) use held::held;
pub(in crate::hardware::broker) use holder::{fresh, Fresh};
pub(crate) use step::step_all;
pub(in crate::hardware::broker) use ways::say_reset_ways;
