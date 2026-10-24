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

use crate::bring::quiet::MASTER_STUCK;
use crate::bring::reset::{AUTO_READ_LATE, RESET_STUCK};
use crate::bring::rings::rx::RX_NOT_ENABLED;
use crate::bring::rings::rx_off::RX_NOT_DISABLED;
use crate::bring::rings::tx::TX_NOT_ENABLED;
use crate::facts::bounds::*;

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
fn the_bounds_are_the_fact_sheet_values() {
    assert_eq!(MASTER_DISABLE_MS, 80, "master stop");
    assert_eq!(SECRX_MS, 40, "security block idle");
    assert_eq!(QUEUE_ENABLE_MS, 20, "queue enable read-back");
    assert_eq!(RESET_SETTLE_MS, 50, "settle after each reset");
    const { assert!(RESET_CLEAR_MS >= 2, "a reset clears in about a millisecond") };
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
