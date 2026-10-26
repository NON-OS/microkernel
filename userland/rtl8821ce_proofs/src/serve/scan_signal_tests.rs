// NONOS Operating System (AGPL-3.0-or-later)

//! The background scan lists each network with the signal the card measured
//! on its beacon, on the panel's 0 to 100 scale, and with none where the card
//! wrote no PHY status.

use nonos_wifi_core::scan_list::signal_percent;

use super::host_radio::host_radio;
use super::radio::Radio;
use super::scanner::Scanner;

const PHYST: u32 = 1 << 26;
const DRV_INFO_32: u32 = 4 << 16;

fn beacon(id: u8) -> Vec<u8> {
    let addr = [0x02, 0, 0, 0, 0, id];
    let head = [&[0x80, 0, 0, 0][..], &[0xFF; 6], &addr, &addr, &[0, 0]].concat();
    [head, vec![0; 8], vec![100, 0, 0x01, 0], vec![0, 3], b"net".to_vec(), vec![0; 4]].concat()
}

#[test]
fn a_scanned_network_carries_its_measured_signal() {
    let (mut radio, mut host) = host_radio();
    let Radio::Up(up) = &mut radio else { unreachable!() };
    let mut phy = vec![0u8; 32];
    phy[..4].copy_from_slice(&(1u32 | 40 << 8).to_le_bytes());
    let b = beacon(1);
    host.stage(b.len() as u32 | DRV_INFO_32 | PHYST, &[phy, b].concat());
    let b = beacon(2);
    host.stage(b.len() as u32, &b);
    let mut scanner = Scanner::new();
    scanner.drain(&mut up.link, &up.regs);
    let mut out = [0u8; 256];
    scanner.cache().encode(&mut out);
    assert_eq!(out[0], 2, "both listed");
    let signals = [out[1], out[1 + 3 + 3]];
    assert!(signals.contains(&signal_percent(-70)), "-70 dBm on the panel's scale");
    assert!(signals.contains(&0), "and none measured for the other");
}

#[test]
fn a_cck_beacon_is_read_with_the_boards_gain_table() {
    let (mut radio, mut host) = host_radio();
    let Radio::Up(up) = &mut radio else { unreachable!() };
    let mut phy = vec![0u8; 32];
    phy[12..16].copy_from_slice(&(1u32 << 23 | 1 << 13 | 3 << 8).to_le_bytes());
    let b = beacon(1);
    let slot = [phy, b.clone()].concat();
    for rfe in [0, 2] {
        host.stage(b.len() as u32 | DRV_INFO_32 | PHYST, &slot);
        up.link.set_rfe(rfe);
        let heard = up.link.poll_heard(&mut [0u8; 2048]).map(|(_, dbm)| dbm);
        let want = if rfe == 0 { -120 } else { -24 - 6 };
        assert_eq!(heard, Some(Some(want)), "front end option {rfe}");
    }
}
