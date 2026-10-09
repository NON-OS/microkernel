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

//! The reset on the clock: Linux rtl_hw_reset's 100 polls of 100 us become
//! a 10 ms deadline, and the stop each version needs comes first.

use std::time::Instant;

use nonos_libc::logged;

use super::model::{live, window};
use super::model_fifo::draining_part;
use crate::chip::MacVersion;
use crate::hw::regs::{MISC_RXDV_GATED_EN, REG_MISC};
use crate::init::reset_run;
use crate::regs::Regs;

#[test]
fn a_reset_that_never_completes_is_given_up_on_after_ten_ms_by_the_clock() {
    let bar = window();
    let start = Instant::now();
    let err = reset_run(&Regs::new(bar.base()), MacVersion(2)).unwrap_err();
    let took = start.elapsed().as_millis();
    assert_eq!(err, "rtl8169 reset timeout");
    assert!((10..1000).contains(&took), "gave up after {took} ms");
    let last = logged().last().cloned().unwrap_or_default();
    assert_eq!(last, "rtl8169: reset did not complete in 10 ms, not started\n");
}

#[test]
fn an_8168g_is_gated_and_drained_before_its_reset() {
    let bar = window();
    let _part = live(&bar, draining_part);
    reset_run(&Regs::new(bar.base()), MacVersion(40)).expect("reset completes");
    assert_ne!(bar.wrote32(REG_MISC) & MISC_RXDV_GATED_EN, 0, "RXDV gate on before reset");
    assert!(logged().iter().all(|l| !l.contains("did not stop")), "{:?}", logged());
}

#[test]
fn an_8168h_whose_fifo_never_drains_is_logged_and_still_reset() {
    let bar = window();
    let _part = live(&bar, super::model::resetting_part);
    reset_run(&Regs::new(bar.base()), MacVersion(46)).expect("reset completes");
    let want = "rtl8169: DMA did not stop before reset (TxConfig TXCFG_EMPTY), resetting anyway\n";
    assert!(logged().iter().any(|l| l == want), "{:?}", logged());
}
