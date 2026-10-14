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
//! W1 (c) on the Realtek link: a link that ended unheard says it was lost, a
//! request to leave says left, and a join's result is counted, so net.core
//! rejoins a lost link and never one the person left.

use nonos_wifi_core::link_history::{Ended, LinkHistory};
use nonos_wifi_core::station::{PROBES, PROBE_EVERY_MS, QUIET_MS};

use super::rig::link;

#[test]
fn a_link_gone_unheard_is_lost_and_a_leave_is_left() {
    let (mut link, _tx) = link();
    assert_eq!(link.history(), LinkHistory::NEW);
    link.note_join(0);
    link.associate(super::rig::AP, [0x5A; 16]);
    link.set_ssid(b"home");
    link.tick(0);
    for n in 0..=u64::from(PROBES) {
        link.tick(QUIET_MS + n * PROBE_EVERY_MS);
    }
    assert_eq!(link.history().ended, Ended::Lost);
    link.note_left();
    assert_eq!(link.history().ended, Ended::Left);
    let h = link.history();
    assert_eq!((h.joins, h.code), (1, 0), "one join, joined");
    link.note_join(-6);
    assert_eq!(
        (link.history().joins, link.history().code, link.history().ended),
        (2, -6, Ended::Left)
    );
}
