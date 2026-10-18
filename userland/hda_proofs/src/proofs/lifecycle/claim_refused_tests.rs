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

//! What the broker may refuse while the controller is taken. A command
//! word or a register window refused ends the attempt with the claim given
//! straight back, and bus mastering never on, which the claim used to turn
//! on first, before anything could refuse it. An interrupt refused in every
//! form ends nothing: the driver polls, and cannot leave before the handles
//! that give the claim back exist.

use nonos_libc::{calls, refuse_command, watch, MK_PCI_CMD_BUS_MASTER};

use crate::error::HdaError;
use crate::proofs::fixtures::{alc236_hp, sim_codec};
use crate::proofs::lifecycle::attempt::{attempt, survey};
use crate::proofs::lifecycle::board::board;
use crate::proofs::lifecycle::claim_tests::bus_master_on;
use crate::proofs::lifecycle::teardown_log::release;
use crate::setup::run;

#[test]
fn a_register_window_refused_after_the_claim_is_let_go_with_bus_mastering_never_on() {
    // No board: the broker has no register window to map.
    watch(|| vec![nonos_libc::command() as u32]);
    let before = calls().len();
    let r = run(&survey(), &mut None);
    let log = calls().split_off(before);
    assert_eq!(r.err(), Some(HdaError::BrokerCallFailed(-19)));
    assert!(bus_master_on(&log).is_empty(), "bus mastering turned on: {log:?}");
    let on = release(&log).regs[0] as u16 & MK_PCI_CMD_BUS_MASTER;
    assert_eq!(on, 0, "bus mastering on at the release");
}

#[test]
fn a_command_word_refused_at_the_claim_gives_the_claim_straight_back() {
    refuse_command();
    let before = calls().len();
    let r = run(&survey(), &mut None);
    let names: Vec<&str> = calls()[before..].iter().map(|c| c.name).collect();
    assert_eq!(r.err(), Some(HdaError::BrokerCallFailed(-1)));
    assert_eq!(names, ["device_claim", "pci_config_read", "device_release"]);
}

#[test]
fn an_interrupt_refused_in_every_form_leaves_the_attempt_polled_and_the_release_last() {
    // The shim's broker refuses every bind; no DMA is offered, so the
    // attempt ends at the rings, once it is past the interrupt.
    let (r, log) = attempt(&board(vec![(0, sim_codec(&alc236_hp()))], &[]));
    assert_eq!(r.err(), Some(HdaError::BrokerCallFailed(-12)), "{log:?}");
    assert!(log.iter().any(|c| c.name == "irq_bind"), "no interrupt was asked for");
    release(&log);
}
