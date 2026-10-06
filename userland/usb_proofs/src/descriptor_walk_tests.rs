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

//! The configuration descriptor walk over lengths the device chose: every
//! record's bLength, the header's wTotalLength, and records cut short. The
//! walk must end, read nothing past wTotalLength or the buffer, and bind
//! only a HID interface's interrupt IN endpoint.

use crate::config_blob::{config, keyboard, summaries, HID, INTERRUPT};
use crate::descriptors::hid_bindings;
use crate::descriptors::types::HidKind;
use crate::protocol::MAX_HID_BINDINGS;

const KBD: (HidKind, u8, u8, u16) = (HidKind::Keyboard, 0, 0x81, 8);

#[test]
fn a_keyboard_and_a_mouse_bind_in_descriptor_order() {
    let raw = keyboard(0).iface(1, HID, 1, 2).hid().ep(0x82, INTERRUPT, 4, 10).build();
    let got = summaries(&hid_bindings(&raw).expect("well formed"));
    assert_eq!(got, vec![KBD, (HidKind::Mouse, 1, 0x82, 4)]);
}

#[test]
fn a_blength_of_zero_or_one_is_refused_rather_than_looped_on() {
    for len in [0u8, 1] {
        let raw = keyboard(0).raw(&[len, 0x24]).build();
        assert_eq!(hid_bindings(&raw).map(|b| b.len()), Err(()), "bLength {len}");
    }
}

#[test]
fn a_blength_past_the_bytes_left_is_refused() {
    let base = keyboard(0);
    let left = 4u8;
    // A record that claims one byte more than wTotalLength leaves it.
    let raw = base.raw(&[left + 1, 0x24, 0, 0]).build();
    assert_eq!(hid_bindings(&raw).map(|b| b.len()), Err(()));
    // A record whose bLength is exactly the bytes left is walked.
    let raw = keyboard(0).raw(&[left, 0x24, 0, 0]).build();
    assert_eq!(summaries(&hid_bindings(&raw).expect("fits")), vec![KBD]);
}

#[test]
fn every_wtotallength_against_one_buffer_is_decided_without_reading_past_it() {
    let blob = keyboard(0);
    let len = blob.size();
    let raw = blob.build();
    for total in 0..=u16::MAX {
        let mut d = raw.clone();
        d[2..4].copy_from_slice(&total.to_le_bytes());
        let got = hid_bindings(&d);
        let total = total as usize;
        if total < 9 || total > len {
            assert!(got.is_err(), "wTotalLength {total} of a {len} byte buffer");
        } else if total == len {
            assert_eq!(summaries(&got.expect("whole")), vec![KBD]);
        } else {
            // Shorter than the buffer: the walk stops at wTotalLength, and
            // the endpoint cut by it is never read.
            assert!(got.map_or(true, |b| b.is_empty()), "wTotalLength {total}");
        }
    }
}

#[test]
fn bytes_past_wtotallength_are_never_walked() {
    let keyboard_len = keyboard(0).size();
    // A whole second HID interface past the end is not bound, and a
    // zero-length record past the end is not seen.
    let raw = keyboard(0)
        .iface(1, HID, 1, 2)
        .hid()
        .ep(0x82, INTERRUPT, 4, 10)
        .raw(&[0, 0])
        .build_with_total(keyboard_len as u16);
    assert_eq!(summaries(&hid_bindings(&raw).expect("first part")), vec![KBD]);
}

#[test]
fn a_single_byte_left_after_the_last_record_ends_the_walk() {
    for b in 0..=u8::MAX {
        let raw = keyboard(0).raw(&[b]).build();
        assert_eq!(summaries(&hid_bindings(&raw).expect("one stray byte")), vec![KBD]);
    }
}

#[test]
fn a_truncated_endpoint_is_stepped_over_and_never_bound() {
    for len in 2u8..7 {
        let mut rec = vec![len, 0x05];
        rec.resize(len as usize, 0x81);
        let raw = config().iface(0, HID, 1, 1).raw(&rec).build();
        assert_eq!(hid_bindings(&raw).map(|b| b.len()), Ok(0), "bLength {len}");
    }
}

#[test]
fn a_truncated_interface_ends_the_interface_before_it() {
    // A keyboard, then an interface record cut short, then an interrupt IN
    // endpoint. The endpoint belongs to the cut record, which names no
    // class, so it is not bound as a second keyboard endpoint.
    for len in 2u8..9 {
        let mut rec = vec![len, 0x04];
        rec.resize(len as usize, HID);
        let raw = keyboard(0).raw(&rec).ep(0x82, INTERRUPT, 8, 10).build();
        let got = summaries(&hid_bindings(&raw).expect("walked"));
        assert_eq!(got, vec![KBD], "interface bLength {len}");
    }
}

#[test]
fn a_header_that_is_not_a_configuration_is_refused() {
    let raw = keyboard(0).build();
    for len in 0..9 {
        assert!(hid_bindings(&raw[..len]).is_err(), "{len} byte buffer");
    }
    for header_len in 0u8..9 {
        let mut d = raw.clone();
        d[0] = header_len;
        assert!(hid_bindings(&d).is_err(), "header bLength {header_len}");
    }
    for ty in (0..=u8::MAX).filter(|&t| t != 0x02) {
        let mut d = raw.clone();
        d[1] = ty;
        assert!(hid_bindings(&d).is_err(), "header type {ty:#x}");
    }
}

#[test]
fn every_short_buffer_is_refused() {
    assert!(hid_bindings(&[]).is_err());
    for a in 0..=u8::MAX {
        assert!(hid_bindings(&[a]).is_err());
        for b in 0..=u8::MAX {
            assert!(hid_bindings(&[a, b]).is_err());
        }
    }
}

#[test]
fn every_record_head_after_a_header_is_walked_without_a_panic() {
    // Every bLength and type for one record, padded with each of three fill
    // bytes, behind a valid header: the walk reads only what is there.
    for len in 0..=u8::MAX {
        for ty in 0..=u8::MAX {
            for fill in [0x00u8, 0x81, 0xff] {
                let mut rec = vec![len, ty];
                rec.resize(2 + (len as usize % 13), fill);
                let _ = hid_bindings(&config().raw(&rec).build());
            }
        }
    }
}

#[test]
fn an_endpoint_before_any_interface_is_not_bound() {
    let raw = config().ep(0x81, INTERRUPT, 8, 10).iface(0, HID, 1, 1).build();
    assert_eq!(hid_bindings(&raw).map(|b| b.len()), Ok(0));
}

#[test]
fn only_a_hid_class_interface_binds() {
    for class in (0..=u8::MAX).filter(|&c| c != HID) {
        let raw = config().iface(0, class, 1, 1).ep(0x81, INTERRUPT, 8, 10).build();
        assert_eq!(hid_bindings(&raw).map(|b| b.len()), Ok(0), "class {class:#x}");
    }
}

#[test]
fn only_an_interrupt_in_endpoint_binds() {
    for attributes in 0..=u8::MAX {
        for address in [0x01u8, 0x81] {
            let raw = config().iface(0, HID, 1, 1).ep(address, attributes, 8, 10).build();
            let bound = hid_bindings(&raw).expect("well formed").len();
            let interrupt_in = address & 0x80 != 0 && attributes & 0x03 == INTERRUPT;
            assert_eq!(bound, usize::from(interrupt_in), "{address:#x} {attributes:#x}");
        }
    }
}

#[test]
fn a_boot_interface_of_no_known_protocol_is_not_bound() {
    for protocol in 0..=u8::MAX {
        let raw = config().iface(0, HID, 1, protocol).ep(0x81, INTERRUPT, 8, 10).build();
        let kinds: Vec<_> = hid_bindings(&raw).expect("ok").iter().map(|b| b.kind).collect();
        let want = match protocol {
            1 => vec![HidKind::Keyboard],
            2 => vec![HidKind::Mouse],
            _ => vec![],
        };
        assert_eq!(kinds, want, "protocol {protocol}");
    }
}

#[test]
fn the_packet_size_is_read_without_the_high_bandwidth_bits() {
    let raw = config().iface(0, HID, 1, 1).ep(0x81, INTERRUPT, 0x1808, 10).build();
    assert_eq!(summaries(&hid_bindings(&raw).expect("ok")), vec![KBD]);
}

#[test]
fn endpoint_zero_is_never_bound() {
    // Endpoint 0 is the control pipe; a record naming it as an interrupt IN
    // endpoint would have the controller driver reconfigure that pipe.
    for address in (0x80u8..=0xff).filter(|a| a & 0x0f == 0) {
        let raw = config().iface(0, HID, 1, 1).ep(address, INTERRUPT, 8, 10).build();
        assert_eq!(hid_bindings(&raw).map(|b| b.len()), Ok(0), "address {address:#x}");
    }
    for number in 1u8..=15 {
        let raw = config().iface(0, HID, 1, 1).ep(0x80 | number, INTERRUPT, 8, 10).build();
        assert_eq!(hid_bindings(&raw).map(|b| b.len()), Ok(1), "endpoint {number}");
    }
}

#[test]
fn a_packet_size_binds_only_when_it_holds_one_report_and_fits_an_interrupt_endpoint() {
    // Keyboard reports are 8 bytes, mouse reports at least 3, tablet
    // reports at least 5; no interrupt endpoint moves more than 1024.
    for (subclass, protocol, least) in [(1u8, 1u8, 8u16), (1, 2, 3), (0, 0, 5)] {
        for wmax in 0..=u16::MAX {
            let raw =
                config().iface(0, HID, subclass, protocol).ep(0x81, INTERRUPT, wmax, 10).build();
            let size = wmax & 0x07ff;
            let want = usize::from((least..=1024).contains(&size));
            let got = hid_bindings(&raw).expect("well formed").len();
            assert_eq!(got, want, "protocol {protocol} wMaxPacketSize {wmax:#x}");
        }
    }
}

#[test]
fn bindings_stop_at_the_cap_however_many_interfaces_follow() {
    let mut blob = config();
    for n in 0..40u8 {
        blob = blob.iface(n, HID, 1, 1).ep(0x81, INTERRUPT, 8, 10);
    }
    let got = hid_bindings(&blob.build()).expect("ok");
    assert_eq!(got.len(), MAX_HID_BINDINGS);
    let numbers: Vec<u8> = got.iter().map(|b| b.interface_number).collect();
    assert_eq!(numbers, (0..MAX_HID_BINDINGS as u8).collect::<Vec<_>>());
}

#[test]
fn the_largest_wtotallength_walks_a_full_buffer_of_padding() {
    // 0xFFFF bytes of two-byte records: the walk ends at the buffer's end.
    let mut d = vec![9, 0x02, 0xff, 0xff, 1, 1, 0, 0xa0, 50];
    while d.len() + 2 <= u16::MAX as usize {
        d.extend_from_slice(&[2, 0x24]);
    }
    d.resize(u16::MAX as usize, 0);
    assert_eq!(hid_bindings(&d).map(|b| b.len()), Ok(0));
}
