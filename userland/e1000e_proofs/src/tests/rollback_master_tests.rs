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

//! An older part whose bus mastering will not stop when a setup fails on a
//! DMA grant: the rollback waits the whole bound for it, says so, and then
//! resets anyway, as Linux does, with Bus Master already off.

use std::time::{Duration, Instant};

use nonos_libc::{clear_log, logged, seen, set_command, watch};
use nonos_libc::{MK_PCI_CMD_BUS_MASTER as BM, MK_PCI_CMD_MEMORY_SPACE as MEM};

use super::order::{no_reset_while_mastering, released_quiet};
use super::probe::{probe, resets};
use super::rollback_tests::grant;
use crate::constants::regs::REG_STATUS;
use crate::constants::status::STATUS_GIO_MASTER_ENABLE;
use crate::constants::timeouts::MASTER_DISABLE_MS;
use crate::constants::Family;
use crate::model::live::live;
use crate::model::part::Behaviour;
use crate::model::window::{i82574, phy_at};
use crate::setup::rollback::after;

#[test]
fn an_older_part_still_mastering_is_waited_for_then_reset_off_the_bus() {
    let bar = i82574();
    bar.present32(REG_STATUS, STATUS_GIO_MASTER_ENABLE);
    let part = live(&bar, phy_at(1), Behaviour::default());
    set_command(MEM | BM);
    watch(probe(&bar, &part.how));
    clear_log();
    let start = Instant::now();
    after(9, 1, &grant(&bar), Family::I82574);
    let took = start.elapsed();
    assert!(took >= Duration::from_millis(MASTER_DISABLE_MS), "gave up after {took:?}");
    let line = "e1000e: PCIe master requests still pending after 80 ms, resetting anyway";
    assert!(logged().iter().any(|l| l == line), "{:?}", logged());
    assert_eq!(resets(&part.how), 1, "reset anyway, as Linux does");
    let seen = seen();
    no_reset_while_mastering(&seen, 1);
    released_quiet(&seen, 1);
}
