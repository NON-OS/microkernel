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

//! Every error line that quotes a bound quotes the bound the code waits, and
//! every step's error fits a console line whole.

use crate::bring::phy::{cycle, lock, owner, word};
use crate::bring::quiet::MASTER_STUCK;
use crate::bring::reset::AUTO_READ_LATE;
use crate::bring::rings::rx::RX_NOT_ENABLED;
use crate::bring::rings::tx::TX_NOT_ENABLED;
use crate::facts::bounds::*;

fn quotes(text: &str, ms: u64) -> bool {
    text.contains(&format!(" {ms} ms"))
}

#[test]
fn quoted_bounds_match_the_waits() {
    assert!(quotes(MASTER_STUCK, MASTER_DISABLE_MS), "{MASTER_STUCK}");
    assert!(quotes(AUTO_READ_LATE, AUTO_READ_MS), "{AUTO_READ_LATE}");
    assert!(quotes(TX_NOT_ENABLED, QUEUE_ENABLE_MS), "{TX_NOT_ENABLED}");
    assert!(quotes(RX_NOT_ENABLED, QUEUE_ENABLE_MS), "{RX_NOT_ENABLED}");
    assert!(quotes(cycle::READ_SLOW, MDIC_MS));
    assert!(quotes(cycle::WRITE_SLOW, MDIC_MS));
}

#[test]
fn the_bounds_are_linux_loop_counts_times_the_longest_sleep() {
    assert_eq!(MASTER_DISABLE_MS * 1000, 800 * 100, "MASTER_DISABLE_TIMEOUT x 100 us");
    assert_eq!(AUTO_READ_MS, 10 * 2, "AUTO_READ_DONE_TIMEOUT x 2 ms");
    assert_eq!(SWFW_SYNC_MS, 200 * 5, "200 x mdelay(5)");
    const { assert!(MDIC_MS * 1000 >= 1920 * 50, "E1000_GEN_POLL_TIMEOUT x 50 us") };
    const { assert!(SEMAPHORE_MS * 1000 >= (32768 + 1) * 50, "a 32K-word NVM x 50 us") };
}

#[test]
fn every_step_error_fits_one_console_line() {
    let all = [
        MASTER_STUCK,
        AUTO_READ_LATE,
        TX_NOT_ENABLED,
        RX_NOT_ENABLED,
        cycle::READ_SLOW,
        cycle::READ_FAILED,
        cycle::WRITE_SLOW,
        cycle::WRITE_FAILED,
        word::REG_RANGE,
        lock::SMBI_HELD,
        lock::SWESMBI_HELD,
        owner::PHY_HELD,
    ];
    for e in all {
        assert!("igb: ".len() + e.len() < 128, "{e}");
        assert!(!e.contains('\u{2014}'), "no em dash");
    }
    assert!(lock::SMBI_HELD.starts_with("phy semaphore not granted"));
    assert!(owner::PHY_HELD.starts_with("phy semaphore not granted"));
}
