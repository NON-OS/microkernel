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

//! A touchpad whose firmware fills its address in at run time: Intel's
//! reference ASL declares TPD0 with I2cSerialBusV2(0x0000, ...) and patches
//! the address in _INI from NVS, which the kernel cannot run, so the record
//! names the controller and nothing else. On that controller the three
//! addresses touchpads commonly ship at are probed in turn with a one-byte
//! read, and the first device that answers is bound, alone in reach, and
//! named by OP_ACPI_HID. A pad anywhere else is not found.

use nonos_i2cmodel::{BusEvent, Command, Config, Dir};

use super::declared::{here, pad_at, reached, this};
use super::fixture::{bench_with, core};
use crate::setup::settle::settle;
use crate::transaction::Presence;

/// Where HID-over-I2C touchpads commonly ship, in the order they are probed.
const SHIPPED_AT: [u8; 3] = [0x15, 0x2C, 0x10];

fn found_at(pad: u8) -> (u8, Vec<u8>, bool) {
    let b = bench_with(Config::LPSS, pad_at(pad));
    let tpd0 = here(0);
    let s = settle(&b.driver, &this(), &tpd0, Presence::Absent, &[tpd0]);
    (s.addr, reached(&s.reach), s.by_probe)
}

#[test]
fn a_declaration_without_an_address_finds_the_pad_at_each_shipped_address() {
    for pad in SHIPPED_AT {
        assert_eq!(found_at(pad), (pad, vec![pad], true), "pad at {pad:#04x}");
    }
}

#[test]
fn a_pad_at_any_other_address_is_not_found() {
    assert_eq!(found_at(0x4B), (0, vec![], false));
}

#[test]
fn the_probe_reads_one_byte_up_to_the_pad_and_writes_nothing() {
    let b = bench_with(Config::LPSS, pad_at(0x10));
    let tpd0 = here(0);
    let _ = settle(&b.driver, &this(), &tpd0, Presence::Absent, &[tpd0]);
    let trace = core(|c| c.bus().trace().to_vec());
    let reads: Vec<u8> = trace.iter().filter_map(read_start).collect();
    let starts = trace.iter().filter(|e| e.is_start() || e.is_restart()).count();
    assert_eq!((reads.as_slice(), starts), (&SHIPPED_AT[..], 3), "up to the pad, in order");
    assert!(trace.iter().all(|e| e.written().is_none()), "a byte was written: {trace:?}");
    assert_eq!(trace.iter().filter(|e| e.read_back().is_some()).count(), 1, "one byte read");
    // A plain read is the input register's, which the pad takes as no command.
    assert_eq!(b.pad.lock().commands(), [Command::InputRead]);
}

fn read_start(e: &BusEvent) -> Option<u8> {
    match *e {
        BusEvent::Start { addr, dir: Dir::Read, .. } => Some(addr),
        BusEvent::Restart { addr, dir: Dir::Read, .. } => Some(addr),
        _ => None,
    }
}
