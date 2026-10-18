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

//! The legacy bring-up's reset, against a device behind a port
//! grant that takes reads to finish it. A device a dead predecessor left
//! live holds its ring until it reads back reset; `setup::legacy` turns Bus
//! Master on only once `negotiate` has returned (`order_tests`), and
//! `negotiate` writes nothing past the reset until the status reads back
//! zero, or refuses the device when it never does. It used to write zero
//! and go straight on to ACKNOWLEDGE.

use crate::constants::STATUS_FEATURES_OK;
use crate::constants::{RESET_MS, STATUS_ACKNOWLEDGE, STATUS_DRIVER, STATUS_DRIVER_OK};
use crate::init::negotiate;
use crate::regs::{port, Regs, Status};

const LIVE: u8 = STATUS_ACKNOWLEDGE | STATUS_DRIVER | STATUS_FEATURES_OK | STATUS_DRIVER_OK;

/// Whether every access before `at` is the reset or a read.
fn only_the_reset_before(log: &[Status], at: usize) -> bool {
    log[..at].iter().all(|s| matches!(s, Status::Write(0) | Status::Read(_)))
}

/// Whether the reset was the first write and read back zero before any
/// other, and DRIVER_OK was never written: a bring-up as far as Bus Master.
pub(super) fn reset_then_handshaken(log: &[Status]) -> bool {
    let ok = log.iter().any(|s| matches!(s, Status::Write(v) if v & STATUS_DRIVER_OK != 0));
    let zero = log.iter().position(|s| *s == Status::Read(0)).unwrap_or(0);
    log.first() == Some(&Status::Write(0)) && zero > 0 && only_the_reset_before(log, zero) && !ok
}

#[test]
fn a_device_left_live_reads_back_reset_before_anything_else_is_written() {
    for held in [1, 3, RESET_MS as u32 - 1] {
        port(|p| (p.status, p.held, p.log) = (LIVE, held, Vec::new()));
        assert_eq!(negotiate(Regs::pio(1)), Ok(()), "held {held}");
        let log = port(|p| std::mem::take(&mut p.log));
        let zero = log.iter().position(|s| *s == Status::Read(0)).expect("zero read back");
        assert_eq!(log[..zero].iter().filter(|s| **s == Status::Read(LIVE)).count(), held as usize);
        assert!(only_the_reset_before(&log, zero), "written past the reset first: {log:?}");
        assert_eq!(log[zero + 1], Status::Write(STATUS_ACKNOWLEDGE), "held {held}");
    }
}

#[test]
fn a_device_that_never_reads_back_reset_is_refused_before_bus_master() {
    port(|p| (p.status, p.held) = (LIVE, u32::MAX));
    assert_eq!(negotiate(Regs::pio(1)), Err("virtio-rng: device did not reset"));
    let log = port(|p| std::mem::take(&mut p.log));
    assert!(only_the_reset_before(&log, log.len()), "written past the reset: {log:?}");
    port(|p| assert_eq!(p.held, u32::MAX - RESET_MS as u32, "given up by the clock"));
}
