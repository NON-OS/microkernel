// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs that the RTL8821CE `RtlLink` satisfies the shared `LinkPort` contract
//! net_core drives. An unassociated link reports down, offers no MAC and refuses
//! to transmit; once associated it reports its MAC and link up. A transmitted
//! Ethernet frame is framed by the station and laid into the TX ring with the
//! hardware CCMP security type set, so the radio encrypts from the CAM, and the
//! write-index doorbell is rung. A received 802.11 MPDU is pulled off the RX ring
//! and parsed back to the exact Ethernet frame that produced it. Driven through
//! the real `nonos_wifi_core::netif::LinkPort` trait against modeled DMA and a
//! modeled card; only the card actually moving bytes is left for on-silicon
//! bring-up.
//!
//! The staged receive frame is what this chip hands up (rtw88 reads the same
//! layout): the access point's FromDS frame, Protected bit still set, decrypted
//! in place with the CCMP header and MIC left on, the 4-byte FCS after it, and

//! W1 on the Realtek link: ticked from the serving loop, an access point gone
//! quiet is probed on the transmit ring, naming its network, and the link
//! ends for the loop to close the session; a link not associated asks nothing.

use nonos_wifi_core::netif::LinkPort;
use nonos_wifi_core::station::{PROBES, PROBE_EVERY_MS, QUIET_MS};

use super::rig::{count, link, probes};

#[test]
fn a_quiet_access_point_is_probed_on_the_ring_then_the_link_ends() {
    let (mut link, tx) = link();
    link.associate(super::rig::AP, [0x5A; 16]);
    link.set_ssid(b"home");
    link.tick(0);
    link.tick(QUIET_MS - 1);
    assert_eq!(probes(&tx), 0, "not before the quiet");
    for n in 0..u64::from(PROBES) {
        link.tick(QUIET_MS + n * PROBE_EVERY_MS);
        assert_eq!(probes(&tx), n as usize + 1);
    }
    let ssid = [0u8, 4, b'h', b'o', b'm', b'e'];
    assert_eq!(count(&tx, &ssid), PROBES as usize, "each probe names the network");
    assert!(link.link_up() && !link.take_silent());
    link.tick(QUIET_MS + u64::from(PROBES) * PROBE_EVERY_MS);
    assert!(!link.link_up());
    assert!(link.take_silent(), "for the loop to close the session");
    assert!(!link.take_silent() && link.take_left().is_none(), "reported once");
}

#[test]
fn a_link_not_associated_asks_nothing() {
    let (mut link, tx) = link();
    for t in (0..10 * QUIET_MS).step_by(100) {
        link.tick(t);
    }
    assert_eq!(probes(&tx), 0);
    assert!(!link.take_silent());
}
