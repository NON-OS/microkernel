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
use crate::init::reset::global::{AUTO_READ_LATE, RESET_STUCK};
use crate::init::reset::master::MASTER_STUCK;
use crate::init::rx_disable::RX_NOT_DISABLED;
use crate::init::rx_queue::RX_NOT_ENABLED;
use crate::init::tx_queue::TX_NOT_ENABLED;

fn quotes(text: &str, ms: u64) -> bool {
    text.contains(&format!(" {ms} ms"))
}

#[test]
fn quoted_bounds_match_the_waits() {
    assert!(quotes(MASTER_STUCK, MASTER_DISABLE_MS), "{MASTER_STUCK}");
    assert!(quotes(RESET_STUCK, RESET_CLEAR_MS), "{RESET_STUCK}");
    assert!(quotes(AUTO_READ_LATE, AUTO_READ_MS), "{AUTO_READ_LATE}");
    assert!(quotes(TX_NOT_ENABLED, QUEUE_ENABLE_MS), "{TX_NOT_ENABLED}");
    assert!(quotes(RX_NOT_ENABLED, QUEUE_ENABLE_MS), "{RX_NOT_ENABLED}");
    assert!(quotes(RX_NOT_DISABLED, QUEUE_ENABLE_MS), "{RX_NOT_DISABLED}");
}

#[test]
fn the_bounds_are_linux_loop_counts_times_the_longest_sleep() {
    assert_eq!(MASTER_DISABLE_MS * 1000, 800 * 100, "IXGBE_PCI_PRIMARY_DISABLE_TIMEOUT x 100 us");
    assert_eq!(SECRX_MS, 40, "IXGBE_MAX_SECRX_POLL x 1 ms");
    assert_eq!(QUEUE_ENABLE_MS, 10 * 2, "IXGBE_MAX_RX_DESC_POLL x 2 ms");
    assert_eq!(RESET_SETTLE_MS, 50, "msleep(50) after each reset");
    const { assert!(RESET_CLEAR_MS >= 2, "usleep_range(1000, 1200) then 10 x udelay(1)") };
}

#[test]
fn every_step_error_fits_one_console_line() {
    let all = [
        MASTER_STUCK,
        RESET_STUCK,
        AUTO_READ_LATE,
        TX_NOT_ENABLED,
        RX_NOT_ENABLED,
        RX_NOT_DISABLED,
    ];
    for e in all {
        assert!("ixgbe: ".len() + e.len() < 128, "{e}");
        assert!(!e.contains('\u{2014}'), "no em dash");
    }
}
