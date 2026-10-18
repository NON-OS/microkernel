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

//! An abort waits up to five seconds for CRR to drop. A controller that
//! failed, USBSTS reading all ones or showing a Host System Error or Host
//! Controller Error, keeps CRR set until it is reset, so the wait ends at
//! once and the controller is taken as wedged.

use std::sync::Arc;
use std::time::{Duration, Instant};

use nonos_devmodel::{run, FakeBar};

use super::fixture::Fixture;
use crate::constants::{CRCR_LO, USBSTS, USBSTS_HCE, USBSTS_HSE};
use crate::controller::program_command_ring;
use crate::rings::command::Abort;

const CRCR_CRR: u32 = 1 << 3;

#[test]
fn an_abort_on_a_failed_controller_ends_at_once() {
    let fx = Fixture::new();
    for sts in [USBSTS_HSE, USBSTS_HCE, u32::MAX] {
        let mut cmd = fx.command_ring();
        let op = Arc::new(FakeBar::new(0x40));
        program_command_ring(op.base(), &mut cmd);
        // CRR before USBSTS, so the failure is seen with the ring running.
        let _failed = run(&op, move |bar: &FakeBar| {
            bar.present32(CRCR_LO as usize, CRCR_CRR);
            bar.present32(USBSTS as usize, sts);
        });
        while op.wrote32(USBSTS as usize) != sts {
            std::thread::yield_now();
        }
        let started = Instant::now();
        assert_eq!(cmd.abort(), Abort::Stuck, "USBSTS {sts:#x}");
        assert!(started.elapsed() < Duration::from_secs(1), "USBSTS {sts:#x}: waited it out");
        assert!(cmd.wedged(), "USBSTS {sts:#x}");
    }
}
