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

//! An ACPI platform device's interrupt reaches the table's record with its
//! `_CRS` trigger and polarity, and a GSI past a byte is refused whole,
//! never cut to fit `irq_line`.

use super::acpi::platform_decode::decode;
use super::acpi::platform_encode::encode;
use super::acpi::platform_record::record;
use super::acpi::platform_wire::{Platform, ACTIVE_LOW, EDGE, GPIO, HAS_IRQ, SHARED};
use super::device::{DEVICE_FLAG_IRQ_ACTIVE_LOW, DEVICE_FLAG_IRQ_EDGE};
use super::round_trip_tests::platform;

const BOTH: u32 = DEVICE_FLAG_IRQ_EDGE | DEVICE_FLAG_IRQ_ACTIVE_LOW;

fn gpio(irq_flags: u8, gsiv: u32) -> Platform {
    Platform { irq_flags, gsiv, ..platform(GPIO, b"AMDI0030", &[(0x1000, 0x400)], b"\\_SB.GPIO") }
}

#[test]
fn trigger_and_polarity_travel_into_the_record() {
    for (bits, want) in [
        (HAS_IRQ, 0),
        (HAS_IRQ | EDGE, DEVICE_FLAG_IRQ_EDGE),
        (HAS_IRQ | ACTIVE_LOW, DEVICE_FLAG_IRQ_ACTIVE_LOW),
        (HAS_IRQ | EDGE | ACTIVE_LOW | SHARED, BOTH),
    ] {
        let r = record(&gpio(bits, 7), 0);
        assert_eq!((r.flags & BOTH, r.irq_line, r.irq_pin, r.irq_source), (want, 7, 1, 7));
    }
}

#[test]
fn a_gsi_past_a_byte_is_refused_not_cut() {
    let r = record(&gpio(HAS_IRQ | EDGE | ACTIVE_LOW, 0x107), 0);
    assert_eq!((r.irq_line, r.irq_pin, r.irq_source, r.flags & BOTH), (0xFF, 0, 0, 0));
    let r = record(&gpio(HAS_IRQ, 0xFF), 0);
    assert_eq!((r.irq_line, r.irq_pin), (0xFF, 0), "0xFF is irq_line's none");
    let r = record(&gpio(0, 0), 0);
    assert_eq!((r.irq_line, r.irq_pin, r.flags & BOTH), (0xFF, 0, 0));
}

#[test]
fn interrupt_bits_without_an_interrupt_or_unknown_are_refused() {
    assert!(encode(&gpio(HAS_IRQ | EDGE, 9)).is_some());
    assert_eq!(encode(&gpio(EDGE, 0)), None, "a trigger with no interrupt");
    let mut raw = encode(&gpio(HAS_IRQ, 9)).unwrap();
    raw[3] = HAS_IRQ | 0x20;
    assert_eq!(decode(&raw), None, "an unknown interrupt bit");
}
