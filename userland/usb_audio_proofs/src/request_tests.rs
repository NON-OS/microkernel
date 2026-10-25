// NONOS Operating System (AGPL-3.0-or-later)
//! The rate request's bytes and a port's speed in OP_PORT_STATUS data.

use crate::portsc::speed_in;
use crate::uac::{set_rate, RATE_HZ};
use nonos_usbnet::Setup;

#[test]
fn set_cur_sampling_frequency_to_the_endpoint() {
    let (setup, data) = set_rate(0x02, RATE_HZ);
    assert_eq!(setup, Setup::new(0x22, 0x01, 0x0100, 0x0002));
    assert_eq!(data, [0x80, 0xbb, 0x00]);
}

fn port_status(ports: &[(u8, u32)]) -> Vec<u8> {
    let mut d = vec![ports.len() as u8, 0, 0, 0];
    for &(port, portsc) in ports {
        d.extend([port, 0, 0, 0]);
        d.extend(portsc.to_le_bytes());
    }
    d
}

#[test]
fn the_named_ports_speed_from_portsc_bits_13_to_10() {
    // Connected, enabled, powered: CCS, PED, PP, with the speed above them.
    let at = |speed: u32| 0x0000_0203 | speed << 10;
    let d = port_status(&[(1, at(1)), (5, at(3)), (9, at(4))]);
    assert_eq!(speed_in(&d, 5), Some(3));
    assert_eq!(speed_in(&d, 1), Some(1));
    assert_eq!(speed_in(&d, 9), Some(4));
    assert_eq!(speed_in(&d, 2), None);
}

#[test]
fn a_count_past_the_data_finds_nothing() {
    let mut d = port_status(&[(1, 3 << 10)]);
    d[0] = 2;
    assert_eq!(speed_in(&d, 1), None);
    assert_eq!(speed_in(&[], 1), None);
}
