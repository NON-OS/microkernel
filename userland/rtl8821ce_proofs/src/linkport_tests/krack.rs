// NONOS Operating System (AGPL-3.0-or-later)

//! Key reinstallation through this driver's own link (802.11-2020, 12.7.6.4
//! and 12.7.7.3): after the join, a retransmitted or replayed message 3 is
//! answered without a pairwise key handed to the chip, and a group key the
//! link already holds, delivered again, is never handed over a second time.

use nonos_wifi_core::netif::LinkPort;
use nonos_wifi_core::wpa::akm::Akm;

use super::krack_rig::{joined, link_after_join, msg3_kd};
use crate::ap_sim::{group1, gtk_kde, message3, ANONCE, GTK};
use crate::tx::regs::REG_TXBD_IDX_BEQ;

#[test]
fn a_retransmitted_message_3_is_answered_and_hands_no_pairwise_key_to_the_chip() {
    let (_, ptk, _) = joined();
    let again = message3(Akm::Psk, 3, &ANONCE, &ptk, &msg3_kd());
    let (mut link, probe) = link_after_join(&[again]);
    let mut out = [0u8; 1600];
    assert!(link.poll_rx(&mut out).is_none(), "nothing for the stack");
    assert!(probe.wrote(REG_TXBD_IDX_BEQ), "message 4 was answered again");
    assert_eq!(link.pairwise_key(), None, "no pairwise key reinstalled into the CAM");
    assert_eq!((link.group_key(), link.stats().rekeys), (None, 0), "and no group key");
}

#[test]
fn a_replayed_message_3_is_dropped_and_installs_nothing() {
    let (_, ptk, _) = joined();
    let old = message3(Akm::Psk, 2, &ANONCE, &ptk, &msg3_kd());
    let (mut link, probe) = link_after_join(&[old]);
    let mut out = [0u8; 1600];
    assert!(link.poll_rx(&mut out).is_none());
    assert!(!probe.wrote(REG_TXBD_IDX_BEQ), "a replayed counter is not answered");
    assert_eq!((link.pairwise_key(), link.group_key()), (None, None));
}

#[test]
fn a_group_key_the_link_holds_is_handed_to_the_chip_once() {
    let (_, ptk, _) = joined();
    let next = [0x22u8; 16];
    // A rekey, the same key again, then the key the join installed handed
    // back: only the first is a key the link does not already hold.
    let first = group1(Akm::Psk, 3, &ptk, &gtk_kde(2, &next));
    let again = group1(Akm::Psk, 4, &ptk, &gtk_kde(2, &next));
    let joins_own = group1(Akm::Psk, 5, &ptk, &gtk_kde(1, &GTK));
    let (mut link, probe) = link_after_join(&[first, again, joins_own]);
    let mut out = [0u8; 1600];
    assert!(link.poll_rx(&mut out).is_none(), "the ring is drained, nothing for the stack");
    assert!(probe.wrote(REG_TXBD_IDX_BEQ), "each group message 1 was answered");
    assert_eq!(link.group_key(), Some((2, &next)), "the new key, not a reinstalled one");
    assert_eq!(link.stats().rekeys, 1, "one key handed to the chip, not three");
}
