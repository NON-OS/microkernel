// NONOS Operating System (AGPL-3.0-or-later)
//! The requests a device's capabilities and NTB parameters call for, and
//! the sizes they leave in force.

use nonos_usbnet::mock::Call;
use nonos_usbnet::{Bind, Setup};

use crate::device::{calls, Dev};
use crate::ncm::bind;
use crate::spec::ntb_params;

fn out(request: u8, value: u16, data: &[u8]) -> Call {
    Call::Out(Setup::new(0x21, request, value, 0), data.to_vec())
}

#[test]
fn crc_off_and_the_16_bit_format_are_set_in_alternate_0_when_offered() {
    let mut dev = Dev::ncm(0x10);
    dev.params = ntb_params(0x0003, 16384, 16384, (4, 0, 4));
    let bus = dev.bus();
    assert!(matches!(bind(bus.clone(), &dev.found()), Bind::Ours(_)));
    let c = calls(&bus);
    assert_eq!(c[4], out(0x8A, 0, &[]), "SET_CRC_MODE, no CRC");
    assert_eq!(c[5], out(0x84, 0, &[]), "SET_NTB_FORMAT, 16-bit");
    assert_eq!(c[6], Call::Out(Setup::set_interface(1, 1), vec![]));
}

#[test]
fn the_8_byte_input_size_and_the_datagram_size_follow_their_bits() {
    let mut dev = Dev::ncm(0x28);
    dev.max_dgram = 2048;
    let bus = dev.bus();
    assert!(matches!(bind(bus.clone(), &dev.found()), Bind::Ours(_)));
    let c = calls(&bus);
    assert_eq!(c[6], out(0x86, 0, &[0x00, 0x10, 0, 0, 0, 0, 0, 0]), "8-byte form");
    assert_eq!(c[7], Call::In(Setup::new(0xA1, 0x87, 0, 0), 2));
    assert_eq!(c[8], out(0x88, 0, &1514u16.to_le_bytes()), "wMaxSegmentSize");
    dev.max_dgram = 1514;
    let bus = dev.bus();
    let _ = bind(bus.clone(), &dev.found());
    assert!(!calls(&bus).iter().any(|c| matches!(c, Call::Out(s, _) if s.request == 0x88)));
}

#[test]
fn a_device_offering_one_transfer_or_less_is_not_told_a_size() {
    for (in_max, rx) in [(4096, 4096), (3000, 3000), (2048, 2048)] {
        let mut dev = Dev::ncm(0);
        dev.params = ntb_params(1, in_max, 16384, (4, 0, 4));
        let bus = dev.bus();
        let Bind::Ours(nic) = bind(bus.clone(), &dev.found()) else { panic!("not bound") };
        assert_eq!(nic.rx_max, rx);
        assert!(!calls(&bus).iter().any(|c| matches!(c, Call::Out(s, _) if s.request == 0x86)));
    }
}

#[test]
fn a_refused_input_size_fails_only_when_blocks_would_not_fit() {
    let mut dev = Dev::ncm(0);
    dev.refuse = Some((0x86, -32));
    let r = bind(dev.bus(), &dev.found());
    assert!(matches!(r, Bind::Failed("SET_NTB_INPUT_SIZE refused, blocks too large", -32)));
    dev.params = ntb_params(1, 1024, 16384, (4, 0, 4));
    let Bind::Ours(nic) = bind(dev.bus(), &dev.found()) else { panic!("not bound") };
    assert_eq!(nic.rx_max, 1024, "2048 asked, refused, the device's own kept");
}

#[test]
fn a_displaylink_dock_is_not_padded_to_the_full_block() {
    let mut dev = Dev::ncm(0);
    dev.vendor = 0x17e9;
    let Bind::Ours(nic) = bind(dev.bus(), &dev.found()) else { panic!("not bound") };
    assert!(nic.shape.zlp);
}
