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

//! The legacy transport as it ships, over the model register file: the
//! reset that stops the device at teardown is waited for by the clock, the
//! link is the device's word only when it reports one, and the doorbell
//! names the queue.

use super::drive::driver;
use super::harness::Dev;
use crate::constants::{Q_RX, Q_TX, RESET_MS, STATUS_DRIVER_OK};
use crate::regs::{fresh, with, Regs};
use crate::transport::Transport;

#[test]
fn a_reset_is_waited_for_until_the_device_reads_back_zero() {
    for lag in [0, 1, 500, RESET_MS as u32 - 1] {
        fresh(256);
        with(|m| (m.status, m.reset_lag) = (STATUS_DRIVER_OK, lag));
        assert!(Transport::Legacy(Regs).reset(), "lag {lag}");
        with(|m| assert_eq!((m.status, m.reset_lag), (0, 0)));
    }
}

#[test]
fn a_reset_that_never_completes_is_given_up_on_when_the_time_is_spent() {
    fresh(256);
    with(|m| m.reset_lag = u32::MAX);
    assert!(!Transport::Legacy(Regs).reset());
    with(|m| assert_eq!(m.reset_lag, u32::MAX - RESET_MS as u32, "one read per millisecond"));
}

#[test]
fn the_link_is_the_devices_word_only_when_it_reports_one() {
    let (rx, tx) = (Dev::rx(), Dev::tx());
    let mut d = driver(&rx, &tx);
    for (supported, word, up) in [(true, 1, true), (true, 0, false), (false, 0, true)] {
        d.status_supported = supported;
        with(|m| m.link = word);
        assert_eq!(d.link_up(), up, "supported {supported} word {word}");
    }
}

#[test]
fn the_doorbell_names_the_queue() {
    fresh(256);
    Transport::Legacy(Regs).notify(Q_TX);
    Transport::Legacy(Regs).notify(Q_RX);
    with(|m| assert_eq!(m.rung, [Q_TX, Q_RX]));
}
