// NONOS Operating System (AGPL-3.0-or-later)
//! A group rekey, and a session that ends before the port opens.

use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::wpa::akm::Akm;

use crate::ap_sim::{gtk_kde, group1};
use crate::gen3::join::exchange::Progress;
use crate::gen3::join::link::Port;
use crate::gen3::join::run::join;
use crate::gen3_model::{reply, Seen};
use crate::join_rig::*;
use super::drive::run_join;

#[test]
fn a_group_rekey_is_answered_protected_and_moves_the_key_in_the_firmware() {
    let r = rig(Security::Wpa2, Switches::default());
    run_join(&r, JoinPolicy::ANY, |fw, out, _| {
        let mut j = out.expect("joined");
        let new_gtk = [0x77u8; 16];
        let ptk = r.ap.borrow().ptk;
        let msg = group1(Akm::Psk, 3, &ptk, &gtk_kde(2, &new_gtk));
        let mut snap = vec![0xAA, 0xAA, 0x03, 0, 0, 0, 0x88, 0x8E];
        snap.extend_from_slice(&msg);
        let pkt = r.ap.borrow_mut().protected(&snap);
        r.model.queue(pkt);
        {
            let mut port = Port { fw, link: &mut j.link };
            port.service();
        }
        assert_eq!(j.link.new_group_key, Some((2, new_gtk)));
        let replies = r.ap.borrow().eapol_replies.len();
        assert_eq!(replies, 3, "messages 2 and 4, then group message 2");
        let last = r.sent().last().unwrap().clone();
        assert!(decrypts(&last.frame, &r.ap.borrow().tk()), "answered protected");
        let (id, key) = j.link.new_group_key.take().unwrap();
        j.keys.rekey(fw, &key, id).expect("installed");
        assert_eq!(j.keys.group, Some(2));
    });
    let keys = r.command_payloads(0x5, 0x18);
    assert_eq!((keys[2][0], keys[2][8], &keys[2][16..32]), (1, 2, &[0x77u8; 16][..]), "the new key at index 2");
    assert_eq!((keys[3][0], keys[3][8]), (3, 1), "then the old one at index 1 removed");
}

#[test]
fn a_session_that_ends_before_the_port_opens_is_asked_for_again() {
    let r = rig(Security::Wpa2, Switches::default());
    let beacon = r.ap.borrow().beacon();
    let mut st = Station::new();
    with_fw(&r, &mut st, |fw| {
        // The session ends as soon as it started.
        let mut p = Progress::default();
        let first = std::cell::Cell::new(true);
        r.model.s.borrow_mut().responder = Some(Box::new(move |c: &Seen| match (c.group, c.cmd, c.payload.get(4)) {
            (0x3, 0x05, Some(1)) if first.replace(false) => {
                vec![reply(c, vec![]), session_notif(1, 1), session_notif(1, 0)]
            }
            (0x3, 0x05, Some(1)) => vec![reply(c, vec![]), session_notif(1, 1)],
            _ => vec![reply(c, vec![])],
        }));
        let j = join(fw, &request(JoinPolicy::ANY), &beacon, 6, 3, 3, &mut p).expect("joined");
        assert!(j.link.station.is_associated());
    });
    let sessions = r.command_payloads(0x3, 0x05);
    assert_eq!(sessions.iter().filter(|s| s[4] == 1).count(), 2, "asked for again");
    assert_eq!(sessions.last().unwrap()[4], 3, "cancelled once the port opened");
}
