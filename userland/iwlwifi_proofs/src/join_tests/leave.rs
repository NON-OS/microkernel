// NONOS Operating System (AGPL-3.0-or-later)
//! Leaving, and the access point's deauthentication.

use nonos_wifi_core::netif::LinkPort;
use nonos_wifi_core::rsn::JoinPolicy;

use crate::gen3::join::bss::Stage;
use crate::gen3::join::link::Port;
use crate::gen3::join::run::leave;
use crate::join_rig::*;
use super::drive::run_join;
use super::rig::{FLUSH, JOIN_COMMANDS, KEY, LINK, MAC, PHY, QUEUE, STA_RM};

#[test]
fn leaving_tells_the_access_point_removes_the_keys_and_takes_everything_down() {
    for (sec, protected) in [(Security::Wpa2, false), (Security::Sae { h2e: true }, true)] {
        let r = rig(sec, Switches::default());
        run_join(&r, JoinPolicy::ANY, |fw, out, _| {
            let mut j = out.expect("joined");
            leave(fw, &mut j);
            assert!(!j.link.station.is_associated());
            assert_eq!(j.bss.stage, Stage::Down);
        });
        let ap = r.ap.borrow();
        assert_eq!(ap.heard.iter().filter(|f| f[0] == 0xC0).count(), 1);
        let deauth = ap.heard.iter().find(|f| f[0] == 0xC0).unwrap();
        assert_eq!(deauth[1] & 0x40 != 0, protected, "protected under the pairwise key with MFP");
        let tail = r.commands()[JOIN_COMMANDS.len()..].to_vec();
        assert_eq!(tail, vec![KEY, KEY, MAC, FLUSH, QUEUE, QUEUE, STA_RM, LINK, PHY]);
        let keys = r.command_payloads(0x5, 0x18);
        assert_eq!((keys[2][0], keys[2][8]), (3, 1), "the group key removed");
        assert_eq!((keys[3][0], keys[3][8]), (3, 0), "then the pairwise key");
    }
}

#[test]
fn the_access_points_deauthentication_takes_the_link_down_unless_it_could_be_forged() {
    // Without management frame protection an unprotected one counts.
    let r = rig(Security::Wpa2, Switches::default());
    run_join(&r, JoinPolicy::ANY, |fw, out, _| {
        let mut j = out.expect("joined");
        let mut port = Port { fw, link: &mut j.link };
        r.model.queue(rx(&leave_frame(), 0));
        assert!(port.poll_rx(&mut [0u8; 2048]).is_none());
        assert!(!port.link_up());
        assert_eq!(j.link.left, Some(15));
    });
    // With it, an unprotected one is ignored; one the firmware decrypted
    // (its MIC held) counts.
    let r = rig(Security::Sae { h2e: true }, Switches { fw_decrypts: true, ..Switches::default() });
    run_join(&r, JoinPolicy::ANY, |fw, out, _| {
        let mut j = out.expect("joined");
        let mut port = Port { fw, link: &mut j.link };
        r.model.queue(rx(&leave_frame(), 0));
        let _ = port.poll_rx(&mut [0u8; 2048]);
        assert!(port.link_up(), "a forged deauthentication is ignored");
        let pkt = r.ap.borrow_mut().protected_mgmt(&leave_frame());
        r.model.queue(pkt);
        let _ = port.poll_rx(&mut [0u8; 2048]);
        assert!(!port.link_up());
        assert_eq!(j.link.left, Some(15));
    });
}

fn leave_frame() -> Vec<u8> {
    let mut f = vec![0xC0, 0, 0, 0];
    f.extend_from_slice(&STA);
    f.extend_from_slice(&AP);
    f.extend_from_slice(&AP);
    f.extend_from_slice(&[0, 0, 15, 0]);
    f
}
