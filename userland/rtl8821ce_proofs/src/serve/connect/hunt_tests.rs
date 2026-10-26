// NONOS Operating System (AGPL-3.0-or-later)

//! The hunt fed hostile frames off the air: random bytes, and beacons for the
//! network mutated. It never panics, keeps no more candidates than its bound,
//! and every candidate it keeps is a beacon for the network on a channel the
//! radio may join, whatever channel the frame names.

use nonos_wifi_core::bss::{Candidates, MAX_CANDIDATES};
use nonos_wifi_core::dot11::parse::parse_beacon;
use nonos_wifi_core::rsn::JoinPolicy;

use super::super::host_radio::host_radio;
use super::super::radio::Radio;
use super::hunt::find_candidates;
use crate::fuzz::Rng;
use crate::rx::ring::RX_DESC_COUNT;

const RSN: [u8; 22] = [
    0x30, 20, 1, 0, 0x00, 0x0F, 0xAC, 4, 1, 0, 0x00, 0x0F, 0xAC, 4, 1, 0, 0x00, 0x0F, 0xAC, 2, 0, 0,
];

// A beacon for "net" from `bssid` naming `channel`, its FCS on as the chip
// hands frames up.
fn beacon(bssid: u8, channel: u8) -> Vec<u8> {
    let addr = [0x02, 0, 0, 0, 0, bssid];
    let head = [&[0x80, 0, 0, 0][..], &[0xFF; 6], &addr, &addr, &[0, 0]].concat();
    let fixed = [&[0u8; 8][..], &100u16.to_le_bytes(), &0x0011u16.to_le_bytes()].concat();
    let ies = [&[0, 3][..], b"net", &[3, 1, channel], &RSN].concat();
    [head, fixed, ies, vec![0; 4]].concat()
}

#[test]
fn a_hunt_through_hostile_frames_keeps_only_the_networks_beacons() {
    let (mut radio, mut host) = host_radio();
    let Radio::Up(up) = &mut radio else { unreachable!() };
    let mut rng = Rng::seeded(0x4B17);
    for round in 0..6 {
        let mut found = Candidates::new(b"net", JoinPolicy::ANY);
        for i in 0..RX_DESC_COUNT - 1 {
            let frame = match i % 4 {
                0 => rng.noise(2400),
                1 => {
                    let seed = beacon(rng.byte(), 6);
                    rng.mutate(&seed)
                }
                2 => beacon(i as u8, rng.byte()),
                _ => beacon(i as u8 + round, 6),
            };
            // A third behind a descriptor of random bits, the rest clean.
            let w0 = if i % 3 == 0 { rng.word() as u32 } else { frame.len() as u32 };
            host.stage(w0, &frame);
        }
        find_candidates(&mut up.link, &up.regs, b"net", None, &mut found);
        let mut kept = 0;
        while let Some(c) = found.take_best() {
            let info = parse_beacon(c.frame()).expect("a beacon");
            assert_eq!(info.ssid, b"net", "only the network's");
            assert!((1..=13).contains(&c.channel()), "a 2.4 GHz channel to join on");
            kept += 1;
        }
        assert!((1..=MAX_CANDIDATES).contains(&kept), "{kept} kept");
    }
}
