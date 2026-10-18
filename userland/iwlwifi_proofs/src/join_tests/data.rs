// NONOS Operating System (AGPL-3.0-or-later)
//! Data crosses the open port both ways through the link protocol.

use nonos_wifi_core::netif::{serve, wire, MAX_RESPONSE};
use nonos_wifi_core::rsn::JoinPolicy;

use crate::gen3::join::link::Port;
use crate::gen3::tx_cmd::{IWL_TX_FLAGS_CMD_RATE, IWL_TX_FLAGS_ENCRYPT_DIS};
use crate::join_rig::*;
use super::drive::{netif, run_join, status};

#[test]
fn data_crosses_both_ways_through_the_link_protocol() {
    for fw_decrypts in [false, true] {
        let sw = Switches { fw_decrypts, ..Switches::default() };
        let r = rig(Security::Wpa2, sw);
        run_join(&r, JoinPolicy::ANY, |fw, out, _| {
            let mut j = out.expect("joined");
            let tk = r.ap.borrow().tk();
            let mut port = Port { fw, link: &mut j.link };
            let mut out = vec![0u8; MAX_RESPONSE];
            let n = serve(&netif(wire::OP_LINK_STATUS, 1, &[]), &mut port, &mut out).unwrap();
            assert_eq!((status(&out), out[n - 1]), (0, 1), "link up");
            serve(&netif(wire::OP_MAC_ADDRESS, 2, &[]), &mut port, &mut out).unwrap();
            assert_eq!(&out[24..30], &STA);

            // Out: an IPv4 frame from the stack reaches the access point
            // protected under the pairwise key.
            let mut eth = vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60];
            eth.extend_from_slice(&STA);
            eth.extend_from_slice(&[0x08, 0x00]);
            eth.extend_from_slice(b"dhcp discover");
            serve(&netif(wire::OP_TX_PACKET, 3, &eth), &mut port, &mut out).unwrap();
            assert_eq!(status(&out), 0);
            let last = r.sent().last().unwrap().clone();
            assert!(decrypts(&last.frame, &tk), "protected by the station");
            assert_eq!(last.flags, IWL_TX_FLAGS_CMD_RATE | IWL_TX_FLAGS_ENCRYPT_DIS, "no second encryption");
            assert_eq!(last.rate, 0x4003, "data at 11 Mb/s, the highest basic rate");
            let got = r.ap.borrow().got.last().unwrap().clone();
            assert_eq!(&got[6..], &[&[0x08, 0x00][..], b"dhcp discover"].concat()[..]);

            // In: the access point's protected reply comes up as Ethernet.
            let mut snap = vec![0xAA, 0xAA, 0x03, 0, 0, 0, 0x08, 0x00];
            snap.extend_from_slice(b"dhcp offer");
            let pkt = r.ap.borrow_mut().protected(&snap);
            r.model.queue(pkt);
            let n = serve(&netif(wire::OP_RX_PACKET, 4, &[]), &mut port, &mut out).unwrap();
            assert_eq!(status(&out), 0, "fw_decrypts={fw_decrypts}");
            let len = u32::from_le_bytes(out[24..28].try_into().unwrap()) as usize;
            let frame = &out[28..n];
            assert_eq!(frame.len(), len);
            assert_eq!(&frame[..6], &STA);
            assert_eq!(&frame[6..12], &AP);
            assert_eq!(&frame[12..], &[&[0x08, 0x00][..], b"dhcp offer"].concat()[..]);
            // The same frame again is a replay.
            r.model.queue(r.ap.borrow().deliver_replay());
            serve(&netif(wire::OP_RX_PACKET, 5, &[]), &mut port, &mut out).unwrap();
            assert_eq!(status(&out), wire::STATUS_AGAIN, "a replay is not delivered");
            assert_eq!(j.link.stats.rx_refused, 1);
        });
    }
}
