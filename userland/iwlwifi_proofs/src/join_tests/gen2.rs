// NONOS Operating System (AGPL-3.0-or-later)
//! A 22000-family part (a Qu B-step AX201 on Qu-b0-hr-b0) joins WPA2 and
//! WPA3-SAE in Linux's order, carries data both ways, and keeps sending
//! past its 256-entry ring's wrap, each frame under the 20-byte gen2
//! transmit command with a byte count in dwords.

use nonos_wifi_core::netif::{serve, wire, MAX_RESPONSE};
use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::wpa::akm::Akm;

use super::drive::{joined, netif, only_handshake_frames_left, run_join, status};
use super::rig::JOIN_COMMANDS;
use crate::gen3::join::link::Port;
use crate::join_rig::*;

#[test]
fn a_22000_part_joins_wpa2_and_sae_in_linuxs_order() {
    for (sec, akm) in [(Security::Wpa2, Akm::Psk), (Security::Sae { h2e: true }, Akm::Sae)] {
        let r = rig_gen2(sec, Switches::default());
        assert_eq!(joined(&r).0, akm);
        assert_eq!(r.commands(), JOIN_COMMANDS.to_vec());
        only_handshake_frames_left(&r);
        assert_eq!(&r.command_payloads(0x5, 0x18)[0][16..32], &r.ap.borrow().tk());
    }
}

#[test]
fn data_crosses_both_ways_and_past_the_rings_wrap() {
    let r = rig_gen2(Security::Wpa2, Switches::default());
    run_join(&r, JoinPolicy::ANY, |fw, out, _| {
        let mut j = out.expect("joined");
        let tk = r.ap.borrow().tk();
        let mut port = Port { fw, link: &mut j.link };
        let mut out = vec![0u8; MAX_RESPONSE];
        for i in 0..300u32 {
            let mut eth = vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60];
            eth.extend_from_slice(&STA);
            eth.extend_from_slice(&[0x08, 0x00]);
            eth.extend_from_slice(&i.to_le_bytes());
            serve(&netif(wire::OP_TX_PACKET, i, &eth), &mut port, &mut out).unwrap();
            assert_eq!(status(&out), 0, "frame {i}");
            assert!(decrypts(&r.sent().last().unwrap().frame, &tk), "frame {i} protected");
            let got = r.ap.borrow().got.last().unwrap().clone();
            assert_eq!(&got[8..], &i.to_le_bytes(), "frame {i} reached the access point");
            // Reading for a frame takes the transmit responses in, freeing slots.
            serve(&netif(wire::OP_RX_PACKET, i, &[]), &mut port, &mut out).unwrap();
            assert_eq!(status(&out), wire::STATUS_AGAIN);
        }
        assert_eq!(r.bc_mismatch(), 0, "every byte count entry in dwords");

        let mut snap = vec![0xAA, 0xAA, 0x03, 0, 0, 0, 0x08, 0x00];
        snap.extend_from_slice(b"dhcp offer");
        let pkt = r.ap.borrow_mut().protected(&snap);
        r.model.queue(pkt);
        let n = serve(&netif(wire::OP_RX_PACKET, 400, &[]), &mut port, &mut out).unwrap();
        assert_eq!(status(&out), 0);
        let frame = &out[28..n];
        assert_eq!((&frame[..6], &frame[6..12]), (&STA[..], &AP[..]));
        assert_eq!(&frame[12..], &[&[0x08, 0x00][..], b"dhcp offer"].concat()[..]);
    });
}
