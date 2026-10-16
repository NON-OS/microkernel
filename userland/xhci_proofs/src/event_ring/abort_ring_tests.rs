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

//! A stuck command aborted through CRCR.CA (xHCI 1.2 section 4.6.1.2).

use std::sync::Arc;
use std::time::{Duration, Instant};

use super::fixture::Fixture;
use crate::constants::CRCR_LO;
use crate::controller::{command::run_command::run_command, program_command_ring};
use crate::error::XhciError;
use crate::rings::command::Abort;
use crate::trb::commands::noop_command;
use nonos_devmodel::{run, FakeBar};

const CRCR_CA: u32 = 1 << 2;
const CRCR_CRR: u32 = 1 << 3;

#[test]
fn an_abort_writes_ca_in_the_low_dword_only_and_waits_for_crr() {
    let fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let op = Arc::new(FakeBar::new(0x40));
    program_command_ring(op.base(), &mut cmd);
    op.present32(CRCR_LO as usize + 4, 0xA5A5_A5A5);
    op.present32(CRCR_LO as usize, CRCR_CRR);
    let _hc = run(&op, |bar: &FakeBar| {
        if bar.wrote32(CRCR_LO as usize) & CRCR_CA != 0 {
            bar.present32(CRCR_LO as usize, 0);
        }
    });
    assert_eq!(cmd.abort(), Abort::Stopped, "the ring stopped");
    assert_eq!(op.wrote32(CRCR_LO as usize + 4), 0xA5A5_A5A5, "the high dword is left alone");
}

#[test]
fn a_ring_never_programmed_has_nothing_to_abort() {
    let fx = Fixture::new();
    let mut cmd = fx.command_ring();
    assert_eq!(cmd.abort(), Abort::NotRunning);
}

#[test]
fn a_command_that_never_completes_is_aborted() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let op = Arc::new(FakeBar::new(0x40));
    program_command_ring(op.base(), &mut cmd);
    op.present32(CRCR_LO as usize, CRCR_CRR);
    let _hc = run(&op, |bar: &FakeBar| {
        if bar.wrote32(CRCR_LO as usize) & CRCR_CA != 0 {
            bar.present32(CRCR_LO as usize, 0);
        }
    });
    let started = Instant::now();
    let trb = noop_command(cmd.cycle() != 0);
    let got = run_command(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, trb);
    assert_eq!(got.map(|_| ()), Err(XhciError::CommandCompletionTimeout));
    assert!(started.elapsed() >= Duration::from_secs(5), "Linux's five seconds");
    assert_eq!(op.wrote32(CRCR_LO as usize) & CRCR_CRR, 0, "the abort ran and the ring stopped");
}
