// NONOS Operating System (AGPL-3.0-or-later)
//! Before the port opens only the access point's own handshake is answered,
//! and the target comes from the beacon and the channel it was heard on.

use nonos_wifi_core::rsn::JoinPolicy;

use super::drive::only_handshake_frames_left;
use super::refusals::refused;
use crate::gen3::join::exchange::limits::IDLE_TRIES;
use crate::gen3::join::exchange::Progress;
use crate::gen3::join::run::{join, JoinEnd};
use crate::gen3::join::target::target;
use crate::join_rig::*;

#[test]
fn nothing_but_eapol_leaves_before_the_port_opens_whatever_arrives() {
    let r = rig(Security::Wpa2, Switches { silent_after_m1: true, ..Switches::default() });
    let beacon = r.ap.borrow().beacon();
    let mut st = Station::new();
    with_fw(&r, &mut st, |fw| {
        // Clear data that is not EAPOL, from the access point and from another
        // BSS, arrives during the handshake: none of it is answered.
        let mut snap = vec![0xAA, 0xAA, 0x03, 0, 0, 0, 0x08, 0x06];
        snap.extend_from_slice(&[0; 28]);
        let arp = from_ap(&snap, 9);
        let mut other = eapol_from_ap(&[0u8; 99], 10);
        other[10] ^= 1;
        r.model.queue(rx(&arp, 0));
        r.model.queue(rx(&other, 0));
        let mut p = Progress::default();
        let out = join(fw, &request(JoinPolicy::ANY), &beacon, 6, 3, 3, &mut p);
        assert_eq!(out.err(), Some(JoinEnd::TimedOut));
        assert!(p.data >= 2, "the clear data was received and dropped");
    });
    only_handshake_frames_left(&r);
}

#[test]
fn only_the_access_points_own_handshake_is_answered() {
    // Message 1 from another BSS arrives right after the access point's: one
    // message 2 answers the real one, and its resends are all that follow.
    let (r, e) = refused(
        Security::Wpa2,
        Switches { silent_after_m1: true, stray_m1: true, ..Switches::default() },
    );
    assert_eq!(e, JoinEnd::TimedOut);
    let m2s: Vec<_> = r.sent().into_iter().filter(|s| s.frame[0] == 0x08).collect();
    assert_eq!(m2s.len() as u32, 1 + IDLE_TRIES);
    assert!(
        m2s.iter().all(|s| s.frame == m2s[0].frame),
        "every one answers the access point's message 1"
    );
}

#[test]
fn the_target_is_read_from_the_beacon_and_the_channel_it_was_heard_on() {
    let b = Ap::new(Security::Wpa2, Switches::default()).beacon();
    let t = target(&b, 11).unwrap();
    assert_eq!(
        (t.bssid, t.channel, t.beacon_int, t.dtim_period),
        (AP, 6, 100, 2),
        "the DS element wins"
    );
    assert_eq!((t.rates.cck_ack, t.rates.ofdm_ack, t.rates.mgmt, t.rates.data), (0x0F, 0x15, 0, 3));
    assert_eq!(t.capability, 0x0431);
    // No DS element (a 5 GHz beacon may carry none): the channel it was
    // heard on; heard on none, no target.
    let mut no_ds = b.clone();
    let at = no_ds.windows(3).position(|w| w == [3, 1, 6]).unwrap();
    no_ds.drain(at..at + 3);
    assert_eq!(target(&no_ds, 36).map(|t| t.channel), Some(36));
    assert_eq!(target(&no_ds, 0), None);
    assert_eq!(target(&b[..30], 6), None, "too short for its fixed fields");
}
