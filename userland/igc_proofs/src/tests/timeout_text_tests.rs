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

use crate::constants::timeouts::*;
use crate::init::phy::{mdic, mdic_word, semaphore, swfw};
use crate::init::reset::global::AUTO_READ_LATE;
use crate::init::reset::master::MASTER_STUCK;
use crate::init::rx_queue::RX_NOT_ENABLED;
use crate::init::tx_queue::TX_NOT_ENABLED;

fn quotes(text: &str, ms: u64) -> bool {
    text.contains(&format!(" {ms} ms"))
}

#[test]
fn quoted_bounds_match_the_waits() {
    assert!(quotes(MASTER_STUCK, MASTER_DISABLE_MS), "{MASTER_STUCK}");
    assert!(quotes(AUTO_READ_LATE, AUTO_READ_MS), "{AUTO_READ_LATE}");
    assert!(quotes(TX_NOT_ENABLED, QUEUE_ENABLE_MS), "{TX_NOT_ENABLED}");
    assert!(quotes(RX_NOT_ENABLED, QUEUE_ENABLE_MS), "{RX_NOT_ENABLED}");
    assert!(quotes(mdic::READ_SLOW, MDIC_MS));
    assert!(quotes(mdic::WRITE_SLOW, MDIC_MS));
}

#[test]
fn the_bounds_are_linux_loop_counts_times_the_longest_sleep() {
    assert_eq!(MASTER_DISABLE_MS, 800 * 3, "MASTER_DISABLE_TIMEOUT x 3 ms");
    assert_eq!(AUTO_READ_MS, 10 * 2, "AUTO_READ_DONE_TIMEOUT x 2 ms");
    assert_eq!(SWFW_SYNC_MS, 200 * 5, "200 x mdelay(5)");
    const { assert!(MDIC_MS * 1000 >= 1920 * 50, "IGC_GEN_POLL_TIMEOUT x 50 us") };
}

#[test]
fn every_step_error_fits_one_console_line() {
    let all = [
        MASTER_STUCK,
        AUTO_READ_LATE,
        TX_NOT_ENABLED,
        RX_NOT_ENABLED,
        mdic::READ_SLOW,
        mdic::READ_FAILED,
        mdic::WRITE_SLOW,
        mdic::WRITE_FAILED,
        mdic_word::REG_RANGE,
        semaphore::SMBI_HELD,
        semaphore::SWESMBI_HELD,
        swfw::PHY_HELD,
    ];
    for e in all {
        assert!("igc: ".len() + e.len() < 128, "{e}");
        assert!(!e.contains('\u{2014}'), "no em dash");
    }
    assert!(semaphore::SMBI_HELD.starts_with("phy semaphore not granted"));
    assert!(swfw::PHY_HELD.starts_with("phy semaphore not granted"));
}
