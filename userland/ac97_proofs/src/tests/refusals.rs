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

//! Every refusal gives back what was taken before it, newest first, and
//! nothing else.

use nonos_libc::{calls, script, Script};

use super::util::{up_with, DEVICE};
use crate::model::Options;
use crate::setup::claim::COMMAND_BITS;

pub const HEAD: [(&str, u64); 3] =
    [("claim", DEVICE), ("pci_command", COMMAND_BITS as u64), ("pio_grant", 0)];
pub const PORTS_BACK: [(&str, u64); 3] =
    [("pio_release", 51), ("pio_release", 50), ("device_release", DEVICE)];

pub fn refused(opt: Options) -> Vec<(&'static str, u64)> {
    let (d, _ich) = up_with(opt);
    assert!(d.is_err(), "the bring-up must refuse");
    calls()
}

pub fn with_tail(
    head: &[(&'static str, u64)],
    tail: &[(&'static str, u64)],
) -> Vec<(&'static str, u64)> {
    head.iter().chain(tail).copied().collect()
}

#[test]
fn a_refused_claim_takes_nothing() {
    script(Script { claim: -16, ..Script::default() });
    assert_eq!(refused(Options::default()), vec![("claim", DEVICE)]);
}

#[test]
fn a_refused_command_write_gives_the_claim_back() {
    script(Script { command: -1, ..Script::default() });
    assert_eq!(refused(Options::default()), with_tail(&HEAD[..2], &[("device_release", DEVICE)]));
}

#[test]
fn a_refused_or_short_mixer_window_gives_the_claim_back() {
    script(Script { ports: [0, 64], ..Script::default() });
    assert_eq!(refused(Options::default()), with_tail(&HEAD, &[("device_release", DEVICE)]));
}

#[test]
fn a_mixer_window_too_small_for_the_vendor_id_is_given_back() {
    script(Script { ports: [0x40, 64], ..Script::default() });
    let back = [("pio_release", 50), ("device_release", DEVICE)];
    assert_eq!(refused(Options::default()), with_tail(&HEAD, &back));
}

#[test]
fn a_refused_bus_master_window_gives_the_mixer_window_back() {
    script(Script { ports: [256, 0], ..Script::default() });
    let back = [("pio_grant", 1), ("pio_release", 50), ("device_release", DEVICE)];
    assert_eq!(refused(Options::default()), with_tail(&HEAD, &back));
}
