// NONOS Operating System (AGPL-3.0-or-later)
//! The register path: each width's lanes, enables and shifts as Linux
//! v6.12 r8152.c sends them, a short read refused, and the PHY's page
//! chosen only when it changes.

use nonos_usbnet::mock::{Call, MockBus};
use nonos_usbnet::Setup;

use crate::rtl::ocp::{read, write};
use crate::rtl::reg::{pla16, pla32, pla8, usb16, usb8};

fn echo(word: u32) -> MockBus {
    MockBus::new(Box::new(move |call| match call {
        Call::In(..) => Ok(word.to_le_bytes().to_vec()),
        _ => Ok(vec![]),
    }))
}

fn sent(bus: &MockBus) -> Vec<(Setup, Vec<u8>)> {
    let calls = bus.0.borrow().calls.clone();
    calls
        .into_iter()
        .filter_map(|c| if let Call::Out(s, d) = c { Some((s, d)) } else { None })
        .collect()
}

#[test]
fn each_width_writes_its_lanes_with_their_enables() {
    let cases = [
        (pla8(0xe813), 0x10, 0xe810, 0x0188, [0, 0, 0, 0x10]),
        (pla8(0xe90a), 0x01, 0xe908, 0x0144, [0, 0, 0x01, 0]),
        (usb8(0xd4b0), 0x03, 0xd4b0, 0x0011, [0x03, 0, 0, 0]),
        (pla16(0xe612), 0x0300, 0xe610, 0x01cc, [0, 0, 0x00, 0x03]),
        (usb16(0xd406), 0x0010, 0xd404, 0x00cc, [0, 0, 0x10, 0]),
        (pla16(0xe86c), 0xa000, 0xe86c, 0x0133, [0x00, 0xa0, 0, 0]),
        (pla32(0xc010), 0x0e, 0xc010, 0x01ff, [0x0e, 0, 0, 0]),
    ];
    for (reg, value, at, index, bytes) in cases {
        let mut bus = echo(0);
        write(&mut bus, reg, value).unwrap();
        assert_eq!(sent(&bus), vec![(Setup::new(0x40, 0x05, at, index), bytes.to_vec())]);
    }
}

#[test]
fn a_read_takes_its_lanes_and_only_a_word_read_names_them() {
    let cases = [(pla8(0xe84f), 0xe84c, 0x0100, 0x12), (pla16(0xe612), 0xe610, 0x01cc, 0x1234)];
    for (reg, at, index, want) in
        cases.into_iter().chain([(pla32(0xe610), 0xe610, 0x0100, 0x1234_5678)])
    {
        let mut bus = echo(0x1234_5678);
        assert_eq!(read(&mut bus, reg), Ok(want));
        assert_eq!(bus.0.borrow().calls, vec![Call::In(Setup::new(0xc0, 0x05, at, index), 4)]);
    }
}

#[test]
fn a_read_shorter_than_a_dword_is_refused() {
    let mut bus = MockBus::new(Box::new(|_| Ok(vec![0xff, 0xff])));
    assert_eq!(read(&mut bus, pla16(0xe908)), Err(-5));
}
