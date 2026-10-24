// NONOS Operating System (AGPL-3.0-or-later)

//! The connect stage's answer to a finished join, the driver's own glue
//! (`serve/connect/install.rs`): a join puts the pairwise and group keys in
//! the chip's key cache, associates the link with the PTK, hands it the
//! supplicant (so the AP's next group rekey is answered), and records the
//! session; a group key id past the cache, or a join that did not end
//! associated, records nothing and leaves the MAC with no link.

use super::install::answer;
use super::install_rig::{joined_with, report, SEC_CMD};
use crate::assoc::Outcome;
use crate::constants::regs::REG_BSSID;
use crate::link::RtlKeys;
use crate::tests_all::linkport_tests::rig::AP_MAC;
use nonos_wifi_core::netif::LinkPort;

#[test]
fn a_join_installs_its_keys_associates_the_link_and_records_the_session() {
    let (r, mut link, card) = joined_with(1);
    let (mut keys, mut session) = (RtlKeys::new(card.clone()), None);
    assert_eq!(answer(r, b"net", &mut link, &mut keys, &card, &mut session).code, 0);
    assert!(card.wrote(SEC_CMD), "the keys went to the chip's key cache");
    let s = session.expect("the session is recorded");
    assert_eq!((s.bssid, s.gtk_id), (AP_MAC, 1));
    let mut out = [0u8; 1600];
    assert!(link.poll_rx(&mut out).is_none());
    assert_eq!(link.group_key(), Some((2, &[0x22; 16])), "the supplicant answers the AP's rekey");
}

#[test]
fn a_group_key_past_the_cache_or_no_join_records_nothing_and_drops_the_link() {
    let (r, mut link, card) = joined_with(4);
    let (mut keys, mut session) = (RtlKeys::new(card.clone()), None);
    assert_eq!(answer(r, b"net", &mut link, &mut keys, &card, &mut session).code, -4);
    assert!(session.is_none() && card.wrote(REG_BSSID), "no session, and the MAC has no link");
    let (_, mut link, card) = joined_with(1);
    let (mut keys, mut session) = (RtlKeys::new(card.clone()), None);
    let code =
        answer(report(Outcome::TimedOut), b"net", &mut link, &mut keys, &card, &mut session).code;
    assert!(code != 0 && session.is_none() && card.wrote(REG_BSSID), "code {code}");
    assert!(!card.wrote(SEC_CMD), "no key reached the cache");
}
