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

//! The console's count and its draw's re-check undone (L79), each a knob
//! on the model, over starts where the model as built breaks nothing.

use super::knob::Knob;
use super::mutation_order_tests::breaks;
use super::starts::{Cfg, Sib};
use super::world_check::KERNEL_DRAWS;

#[test]
fn one_flag_for_two_takeovers_lets_the_console_draw_under_the_second() {
    let keep = |c: &Cfg| c.g2 && c.doomed && c.sib == Sib::Idle;
    breaks(Knob::TakersFlag, keep, 100_000, KERNEL_DRAWS);
}

#[test]
fn a_drawer_that_reads_the_stop_only_before_counting_is_not_waited_for() {
    let keep = |c: &Cfg| !c.g2 && !c.doomed && !c.again && c.sib == Sib::Idle;
    breaks(Knob::NoRecheck, keep, 5_000, KERNEL_DRAWS);
}
