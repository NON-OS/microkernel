// NONOS Operating System (AGPL-3.0-or-later)
//! A child or hub that names sizes no device has: an endpoint 0 size the
//! speed does not allow is not parked, the host's park check refuses one
//! too, and a hub's status endpoint of no size or no interval is not
//! polled.

use nonos_usb_hub::{ep0_size, park_body, status_endpoint, Place};
use nonos_usb_hub::{SPEED_FULL, SPEED_HIGH, SPEED_LOW, SPEED_SUPER};
use nonos_usb_virt::parse_park;

#[test]
fn endpoint_0_sizes_are_those_each_speed_allows() {
    assert_eq!(ep0_size(SPEED_SUPER, 9), Some(512), "an exponent at SuperSpeed");
    assert_eq!(ep0_size(SPEED_SUPER, 64), None);
    assert_eq!((ep0_size(SPEED_HIGH, 64), ep0_size(SPEED_HIGH, 8)), (Some(64), None));
    assert_eq!((ep0_size(SPEED_LOW, 8), ep0_size(SPEED_LOW, 64)), (Some(8), None));
    for b in [8, 16, 32, 64] {
        assert_eq!(ep0_size(SPEED_FULL, b), Some(u16::from(b)));
    }
    for b in [0, 7, 9, 63, 255] {
        assert_eq!(ep0_size(SPEED_FULL, b), None, "{b}");
    }
}

#[test]
fn the_host_parks_no_endpoint_0_size_a_device_cannot_have() {
    let place = Place { root_port: 1, route: 0x1, depth: 1, speed: 3, tt: None, parent: (2, 1) };
    assert!(parse_park(&park_body(5, &place, 512)).is_some());
    for mps0 in [0, 9, 255, 1024] {
        assert!(parse_park(&park_body(5, &place, mps0)).is_none(), "{mps0}");
    }
}

fn hub_config(mps: u16, interval: u8) -> Vec<u8> {
    let [m0, m1] = mps.to_le_bytes();
    vec![9, 2, 25, 0, 1, 1, 0, 0xe0, 0, 9, 4, 0, 0, 1, 9, 0, 0, 0, 7, 5, 0x81, 3, m0, m1, interval]
}

#[test]
fn a_status_endpoint_of_no_size_or_no_interval_is_not_polled() {
    assert!(status_endpoint(&hub_config(2, 12)).is_some());
    assert_eq!(status_endpoint(&hub_config(0, 12)), None);
    assert_eq!(status_endpoint(&hub_config(2, 0)), None);
    assert_eq!(status_endpoint(&hub_config(1025, 12)), None);
    assert_eq!(status_endpoint(&hub_config(1 << 11 | 2, 12)), None);
}
