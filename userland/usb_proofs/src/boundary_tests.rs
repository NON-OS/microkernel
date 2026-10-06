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

//! The edges, by name: each case is one input at a boundary of a rule and
//! the one outcome the rule allows there. A failure names its case.

use crate::config_blob::{config, keyboard, summaries, Blob, HID, INTERRUPT};
use crate::descriptors::hid_bindings;
use crate::descriptors::types::HidKind;
use crate::hid::mouse_report::mouse_event;
use crate::hid::tablet_report::tablet_report;
use crate::keyboard_tests::{report, step};

type Binding = (HidKind, u8, u8, u16);
/// A named descriptor and the bindings the walk must return for it.
type ConfigCase = (&'static str, Vec<u8>, Result<Vec<Binding>, ()>);
/// A named keyboard report and the key events it must make.
type KeyCase = (&'static str, Vec<u8>, Vec<(u8, bool)>);

const KBD: Binding = (HidKind::Keyboard, 0, 0x81, 8);

fn with_endpoint(address: u8, size: u16) -> Vec<u8> {
    config().iface(0, HID, 1, 1).hid().ep(address, INTERRUPT, size, 10).build()
}

fn kind_with(subclass: u8, protocol: u8, size: u16) -> Vec<u8> {
    config().iface(0, HID, subclass, protocol).ep(0x81, INTERRUPT, size, 10).build()
}

fn cut(blob: Blob, total: u16) -> Vec<u8> {
    blob.build_with_total(total)
}

#[test]
fn configuration_descriptor_edges() {
    let header_only = config().build();
    let cases: Vec<ConfigCase> = vec![
        ("empty buffer", vec![], Err(())),
        ("eight bytes", header_only[..8].to_vec(), Err(())),
        ("header alone", header_only.clone(), Ok(vec![])),
        ("wTotalLength 8", cut(config(), 8), Err(())),
        ("wTotalLength 0", cut(config(), 0), Err(())),
        ("wTotalLength 0xffff on nine bytes", cut(config(), 0xffff), Err(())),
        ("wTotalLength one past the buffer", cut(keyboard(0), 35), Err(())),
        ("wTotalLength one short of the endpoint", cut(keyboard(0), 33), Err(())),
        ("wTotalLength exact", keyboard(0).build(), Ok(vec![KBD])),
        ("bLength 0", keyboard(0).raw(&[0, 0x24]).build(), Err(())),
        ("bLength 1", keyboard(0).raw(&[1, 0x24]).build(), Err(())),
        ("bLength 2", keyboard(0).raw(&[2, 0x24]).build(), Ok(vec![KBD])),
        ("bLength 255 in 4 bytes", keyboard(0).raw(&[255, 0x24, 0, 0]).build(), Err(())),
        ("one stray byte", keyboard(0).raw(&[7]).build(), Ok(vec![KBD])),
        (
            "interface bLength 8",
            config().raw(&[8, 4, 0, 0, 1, HID, 1, 1]).ep(0x81, 3, 8, 10).build(),
            Ok(vec![]),
        ),
        (
            "endpoint bLength 6",
            config().iface(0, HID, 1, 1).raw(&[6, 5, 0x81, 3, 8, 0]).build(),
            Ok(vec![]),
        ),
        ("endpoint 0 IN", with_endpoint(0x80, 8), Ok(vec![])),
        ("endpoint 1 IN", with_endpoint(0x81, 8), Ok(vec![KBD])),
        ("endpoint 15 IN", with_endpoint(0x8f, 8), Ok(vec![(HidKind::Keyboard, 0, 0x8f, 8)])),
        ("endpoint 1 OUT", with_endpoint(0x01, 8), Ok(vec![])),
        ("keyboard packet 0", with_endpoint(0x81, 0), Ok(vec![])),
        ("keyboard packet 7", with_endpoint(0x81, 7), Ok(vec![])),
        (
            "keyboard packet 1024",
            with_endpoint(0x81, 1024),
            Ok(vec![(HidKind::Keyboard, 0, 0x81, 1024)]),
        ),
        ("keyboard packet 1025", with_endpoint(0x81, 1025), Ok(vec![])),
        ("keyboard packet 0x7ff", with_endpoint(0x81, 0x7ff), Ok(vec![])),
        ("keyboard packet 8, high bandwidth bits", with_endpoint(0x81, 0x1808), Ok(vec![KBD])),
        ("mouse packet 2", kind_with(1, 2, 2), Ok(vec![])),
        ("mouse packet 3", kind_with(1, 2, 3), Ok(vec![(HidKind::Mouse, 0, 0x81, 3)])),
        ("tablet packet 4", kind_with(0, 0, 4), Ok(vec![])),
        ("tablet packet 5", kind_with(0, 0, 5), Ok(vec![(HidKind::Tablet, 0, 0x81, 5)])),
        ("boot subclass, protocol 0", kind_with(1, 0, 8), Ok(vec![])),
        ("boot subclass, protocol 3", kind_with(1, 3, 8), Ok(vec![])),
    ];
    for (name, raw, want) in cases {
        let got = hid_bindings(&raw).map(|b| summaries(&b));
        assert_eq!(got, want, "{name}");
    }
}

#[test]
fn keyboard_report_edges() {
    const A: u8 = 0x04;
    let held = step([0; 6], &report(0, &[A])).1;
    let lone = |k: u8| report(0, &[k]).to_vec();
    let mut short = report(0, &[]).to_vec();
    short.truncate(7);
    let mut long = report(0, &[]).to_vec();
    long.push(0);
    // (case, report fed with A held, key events it makes)
    let cases: Vec<KeyCase> = vec![
        ("all zero", report(0, &[]).to_vec(), vec![(A, false)]),
        ("A held", lone(A), vec![]),
        ("ErrorRollOver in every slot", report(0, &[1; 6]).to_vec(), vec![]),
        ("ErrorRollOver in one slot", report(0, &[A, 1]).to_vec(), vec![]),
        ("POSTFail", report(0, &[2; 6]).to_vec(), vec![]),
        ("ErrorUndefined", report(0, &[3; 6]).to_vec(), vec![]),
        ("first key 0x04 with B", report(0, &[A, 0x05]).to_vec(), vec![(0x05, true)]),
        ("last main key 0xa4", lone(0xa4), vec![(0xa4, true), (A, false)]),
        ("first reserved 0xa5", lone(0xa5), vec![(A, false)]),
        ("last reserved 0xaf", lone(0xaf), vec![(A, false)]),
        ("first keypad extra 0xb0", lone(0xb0), vec![(0xb0, true), (A, false)]),
        ("last keypad extra 0xdd", lone(0xdd), vec![(0xdd, true), (A, false)]),
        ("reserved 0xde", lone(0xde), vec![(A, false)]),
        ("left control usage 0xe0", lone(0xe0), vec![(A, false)]),
        ("right GUI usage 0xe7", lone(0xe7), vec![(A, false)]),
        ("reserved 0xe8", lone(0xe8), vec![(A, false)]),
        ("0xff", lone(0xff), vec![(A, false)]),
        (
            "one key in all six slots",
            report(0, &[0x05; 6]).to_vec(),
            vec![(0x05, true), (A, false)],
        ),
        ("every modifier bit", report(0xff, &[A]).to_vec(), vec![]),
        ("empty report", vec![], vec![]),
        ("seven bytes", short, vec![]),
        ("nine bytes", long, vec![]),
    ];
    for (name, raw, want) in cases {
        assert_eq!(step(held, &raw).0, want, "{name}");
    }
}

#[test]
fn mouse_report_edges() {
    type Got = Option<(i16, i16, i8, u8)>;
    let cases: Vec<(&str, Vec<u8>, Got)> = vec![
        ("empty", vec![], None),
        ("two bytes", vec![1, 1], None),
        ("three quiet bytes", vec![0, 0, 0], None),
        ("most negative", vec![0, 0x80, 0x80], Some((-128, -128, 0, 0))),
        ("most positive", vec![0, 0x7f, 0x7f], Some((127, 127, 0, 0))),
        ("minus one", vec![0, 0xff, 0xff, 0xff], Some((-1, -1, -1, 0))),
        ("wheel most negative", vec![0, 0, 0, 0x80], Some((0, 0, -128, 0))),
        ("every button bit", vec![0xff, 0, 0], Some((0, 0, 0, 0x1f))),
        ("padding bits alone", vec![0xe0, 0, 0], None),
        ("eight bytes", vec![1, 2, 3, 4, 0xff, 0xff, 0xff, 0xff], Some((2, 3, 4, 1))),
    ];
    for (name, raw, want) in cases {
        let got = mouse_event(&raw, 0).map(|e| (e.dx, e.dy, e.dz, e.buttons));
        assert_eq!(got, want, "{name}");
    }
}

#[test]
fn tablet_report_edges() {
    type Got = Option<(i32, i32, i32, u8)>;
    let cases: Vec<(&str, Vec<u8>, Got)> = vec![
        ("four bytes", vec![0, 0, 0, 0], None),
        ("origin", vec![0, 0, 0, 0, 0], Some((0, 0, 0, 0))),
        ("top of the range", vec![0, 0xff, 0x7f, 0xff, 0x7f], Some((0x7fff, 0x7fff, 0, 0))),
        ("one past the top", vec![0, 0x00, 0x80, 0x00, 0x80], Some((0x7fff, 0x7fff, 0, 0))),
        ("0xffff", vec![0, 0xff, 0xff, 0xff, 0xff], Some((0x7fff, 0x7fff, 0, 0))),
        ("wheel most negative", vec![0, 0, 0, 0, 0, 0x80], Some((0, 0, -128, 0))),
        ("every button bit", vec![0xff, 0, 0, 0, 0], Some((0, 0, 0, 0x07))),
        ("eight bytes", vec![1, 1, 0, 2, 0, 3, 0xff, 0xff], Some((1, 2, 3, 1))),
    ];
    for (name, raw, want) in cases {
        let got = tablet_report(&raw).map(|t| (t.x, t.y, t.wheel, t.buttons));
        assert_eq!(got, want, "{name}");
    }
}
