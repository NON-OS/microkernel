// NONOS Operating System (AGPL-3.0-or-later)
//! The inbox is bounded, and a network the station does not join is left
//! untouched.

use nonos_wifi_core::mlme::MlmeFailure;
use nonos_wifi_core::rsn::{JoinPolicy, SelectError};

use crate::gen3::join::inbox::{Inbox, Queues, INBOX_FRAMES};
use crate::gen3::packet::Packet;
use crate::gen3::txq::{TxQueue, TXQ_STRIDE};
use crate::gen3::join::exchange::Progress;
use crate::gen3::join::run::{join, JoinEnd};
use crate::join_rig::*;

#[test]
fn the_inbox_is_bounded_and_routes_transmit_responses_to_their_queue() {
    let mut q = Queues { mgmt: TxQueue::new(0), data: TxQueue::new(TXQ_STRIDE) };
    q.mgmt.start(4, 0);
    let mut inbox = Inbox::new();
    let f = from_ap(&[0xAA, 0xAA, 3, 0, 0, 0, 8, 0], 1);
    let pkt = rx(&f, 0);
    for _ in 0..INBOX_FRAMES + 1 {
        inbox.route(&Packet { cmd: pkt.cmd, group: 0, sequence: 0x8000, payload: &pkt.payload }, &mut q);
    }
    assert_eq!((inbox.frames.len(), inbox.dropped), (INBOX_FRAMES, 1));
    // A response for a queue the access point has not got is stray.
    let mut resp = vec![0u8; 48];
    resp[0] = 1;
    resp[36] = 9;
    resp[40] = 1;
    inbox.route(&Packet { cmd: 0x1C, group: 0, sequence: 0, payload: &resp }, &mut q);
    assert_eq!(inbox.stray, 1);
    resp[36] = 4;
    resp[44] = 5;
    inbox.route(&Packet { cmd: 0x1C, group: 0, sequence: 0, payload: &resp }, &mut q);
    assert_eq!(inbox.stray, 2, "naming a slot not in flight");
}

#[test]
fn networks_the_station_does_not_join_send_nothing_and_touch_no_context() {
    // An open network.
    let r = rig(Security::Wpa2, Switches::default());
    let mut st = Station::new();
    with_fw(&r, &mut st, |fw| {
        let open = beacon(&[]);
        let out = join(fw, &request(JoinPolicy::ANY), &open, 6, 3, 3, &mut Progress::default());
        assert_eq!(out.err(), Some(JoinEnd::Refused(MlmeFailure::OpenNetwork)));
        // Saved as WPA3, now offering WPA2 only.
        let wpa2 = beacon(&[&RSNE_PSK]);
        let out = join(fw, &request(JoinPolicy::WPA3_ONLY), &wpa2, 6, 3, 3, &mut Progress::default());
        assert_eq!(out.err(), Some(JoinEnd::Refused(MlmeFailure::Select(SelectError::Downgrade))));
        // Another network's beacon.
        let mut theirs = wpa2.clone();
        theirs[38] = b'H';
        let out = join(fw, &request(JoinPolicy::ANY), &theirs, 6, 3, 3, &mut Progress::default());
        assert_eq!(out.err(), Some(JoinEnd::NotTheNetwork));
        // The network's beacon, but with no channel in it and none it was
        // heard on.
        let mut nowhere = wpa2.clone();
        let at = nowhere.windows(3).position(|w| w == [3, 1, 6]).unwrap();
        nowhere.drain(at..at + 3);
        let out = join(fw, &request(JoinPolicy::ANY), &nowhere, 0, 3, 3, &mut Progress::default());
        assert_eq!(out.err(), Some(JoinEnd::NotTheNetwork));
    });
    assert!(r.commands().is_empty(), "no firmware context was put up");
    assert!(r.sent().is_empty(), "nothing was sent");
}
