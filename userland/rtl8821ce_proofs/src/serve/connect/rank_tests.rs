// NONOS Operating System (AGPL-3.0-or-later)

//! The hunt ranks the network's access points by the signal the card measured
//! on their beacons: a stronger one heard once comes before a weaker one heard
//! twice, which would win on count alone.

use nonos_wifi_core::bss::Candidates;
use nonos_wifi_core::rsn::JoinPolicy;

use super::super::host_radio::host_radio;
use super::super::radio::Radio;
use super::hunt::find_candidates;

const RSN: [u8; 22] = [
    0x30, 20, 1, 0, 0x00, 0x0F, 0xAC, 4, 1, 0, 0x00, 0x0F, 0xAC, 4, 1, 0, 0x00, 0x0F, 0xAC, 2, 0, 0,
];
const PHYST: u32 = 1 << 26;
const DRV_INFO_32: u32 = 4 << 16;

// A beacon for "net" from `id`, behind 32 bytes of OFDM PHY status at
// `dbm`, its FCS on: the slot as the card fills it, and its descriptor word.
fn heard(id: u8, dbm: i8) -> (u32, Vec<u8>) {
    let addr = [0x02, 0, 0, 0, 0, id];
    let head = [&[0x80, 0, 0, 0][..], &[0xFF; 6], &addr, &addr, &[0, 0]].concat();
    let fixed = [&[0u8; 8][..], &100u16.to_le_bytes(), &0x0011u16.to_le_bytes()].concat();
    let frame =
        [head, fixed, [&[0, 3][..], b"net", &[3, 1, 6], &RSN].concat(), vec![0; 4]].concat();
    let mut phy = vec![0u8; 32];
    phy[..4].copy_from_slice(&(1 | ((i16::from(dbm) + 110) as u32) << 8).to_le_bytes());
    (frame.len() as u32 | DRV_INFO_32 | PHYST, [phy, frame].concat())
}

#[test]
fn the_stronger_access_point_comes_first() {
    let (mut radio, mut host) = host_radio();
    let Radio::Up(up) = &mut radio else { unreachable!() };
    for (id, dbm) in [(1, -80), (1, -80), (2, -40)] {
        let (w0, slot) = heard(id, dbm);
        host.stage(w0, &slot);
    }
    let mut found = Candidates::new(b"net", JoinPolicy::ANY);
    find_candidates(&mut up.link, &up.regs, b"net", None, &mut found);
    let best = found.take_best().expect("both heard");
    assert_eq!(best.bssid()[5], 2, "the -40 dBm access point");
    assert_eq!(found.take_best().map(|c| c.bssid()[5]), Some(1));
}
