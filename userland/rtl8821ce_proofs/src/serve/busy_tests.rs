// NONOS Operating System (AGPL-3.0-or-later)

//! Busy traffic either way wakes a sleeping radio at the period's end, the
//! access point heard half a period before it, so its watch sends nothing.

use nonos_wifi_core::netif::LinkPort;

use super::host_radio::host_radio;
use super::radio::Radio;
use super::sleep_tests::{asleep, T0};
use crate::ps::{BUSY_FRAMES, PERIOD_MS};
use crate::regs::Mmio;
use crate::tests_all::linkport_tests::receive::{chip_decrypted, W0_ENC_AES};
use crate::tests_all::linkport_tests::rig::{eth_frame, AP_MAC, STA_MAC};
use crate::tx::regs::REG_TXBD_IDX_BEQ;

#[test]
fn busy_receive_wakes_it() {
    let (mut radio, mut host) = host_radio();
    let Radio::Up(up) = &mut radio else { unreachable!() };
    asleep(up, &mut host);
    up.link.tick(T0 + PERIOD_MS + PERIOD_MS / 2);
    let eth = eth_frame(STA_MAC, AP_MAC, [0x08, 0x00], &[0x45; 40]);
    for pn in 1..=BUSY_FRAMES as u8 + 1 {
        let f = chip_decrypted(&eth, pn);
        host.stage(f.len() as u32 | W0_ENC_AES, &f);
        assert!(up.link.poll_rx(&mut [0u8; 2048]).is_some(), "frame {pn} delivered");
    }
    up.link.tick(T0 + 2 * PERIOD_MS);
    assert_eq!(up.link.stats().watch_sends, 0, "the access point was heard");
    assert!(!up.link.ps().asleep(), "over the threshold");
}

#[test]
fn busy_transmit_wakes_it() {
    let (mut radio, mut host) = host_radio();
    let Radio::Up(up) = &mut radio else { unreachable!() };
    asleep(up, &mut host);
    up.link.tick(T0 + PERIOD_MS + PERIOD_MS / 2);
    let eth = eth_frame(AP_MAC, STA_MAC, [0x08, 0x00], &[0x45; 40]);
    for sent in 0..BUSY_FRAMES + 1 {
        // The card read every frame so far, so the ring never fills.
        host.regs.write32(REG_TXBD_IDX_BEQ, (sent % 32) << 16);
        assert!(up.link.send_tx(&eth), "frame {sent} sent");
    }
    let beacon = super::sleep_tests::beacon();
    host.stage(beacon.len() as u32, &beacon);
    let _ = up.link.poll_rx(&mut [0u8; 2048]);
    up.link.tick(T0 + 2 * PERIOD_MS);
    assert!(!up.link.ps().asleep(), "over the threshold");
}
