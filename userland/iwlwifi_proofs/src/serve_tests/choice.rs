// NONOS Operating System (AGPL-3.0-or-later)
//! The hunt's candidates in the order the join takes them: better security
//! first, then the stronger signal; and a network heard only as one the
//! policy refuses is no candidate, the refusal kept for the answer.

use nonos_wifi_core::bss::Candidates;
use nonos_wifi_core::mlme::MlmeFailure;
use nonos_wifi_core::rsn::{JoinPolicy, SelectError};

use super::scripted::{completion, heard_at};
use crate::gen3::join::hunt::{hunt, HuntEnd};
use crate::gen3::sweep::Sweep;
use crate::gen3_model::{reply, Seen};
use crate::join_rig::{beacon, rig, Security, Station, Switches, RSNE_PSK};

// WPA3-Personal: SAE, CCMP, management frame protection capable.
const RSNE_SAE: [u8; 22] =
    [48, 20, 1, 0, 0, 0x0F, 0xAC, 4, 1, 0, 0, 0x0F, 0xAC, 4, 1, 0, 0, 0x0F, 0xAC, 8, 0x80, 0];

// Another access point of the same network.
fn other_ap(rsne: &[u8]) -> Vec<u8> {
    let mut b = beacon(&[rsne]);
    b[10] ^= 0x01;
    b[16] ^= 0x01;
    b
}

fn hunted(frames: Vec<(Vec<u8>, u8, u8)>, policy: JoinPolicy) -> (Result<(), HuntEnd>, Candidates) {
    let r = rig(Security::Wpa2, Switches::default());
    r.model.s.borrow_mut().responder = Some(Box::new(move |c: &Seen| match (c.group, c.cmd) {
        (0x1, 0x0D) => {
            let mut out = vec![reply(c, vec![])];
            out.extend(frames.iter().map(|(f, ch, e)| heard_at(f, *ch, *e)));
            out.push(completion(c, 1));
            out
        }
        _ => vec![reply(c, vec![])],
    }));
    let (mut st, mut found) = (Station::new(), Candidates::new(b"home", policy));
    let mut got = Err(HuntEnd::Radio);
    crate::join_rig::with_fw(&r, &mut st, |fw| {
        got = hunt(
            fw.dev,
            fw.clock,
            &mut Sweep::new(),
            &[1, 6, 11],
            &mut found,
            &mut || 0,
            &mut |_| {},
        );
    });
    (got, found)
}

#[test]
fn security_first_then_signal() {
    let (psk_near, psk_far, sae_far) =
        (beacon(&[&RSNE_PSK]), other_ap(&RSNE_PSK), other_ap(&RSNE_SAE));
    let frames =
        vec![(psk_far.clone(), 11, 80), (psk_near.clone(), 1, 30), (sae_far.clone(), 6, 85)];
    let (got, mut found) = hunted(frames, JoinPolicy::ANY);
    assert_eq!(got, Ok(()));
    let order: Vec<Vec<u8>> =
        core::iter::from_fn(|| found.take_best()).map(|c| c.frame().to_vec()).collect();
    assert_eq!(order, vec![sae_far, psk_near, psk_far], "WPA3, then the nearer WPA2");
}

#[test]
fn a_network_heard_only_as_a_downgrade_is_no_candidate() {
    let (got, found) = hunted(vec![(beacon(&[&RSNE_PSK]), 6, 40)], JoinPolicy::WPA3_ONLY);
    assert_eq!(got, Err(HuntEnd::NotHeard(1)));
    assert_eq!(found.refusal(), Some(MlmeFailure::Select(SelectError::Downgrade)));
}
