// NONOS Operating System (AGPL-3.0-or-later)
//! A receive buffer's packet, and the notifications and replies in it:
//! alive, NVM, TX response, queue reply, session, scan completion and MCC.

use crate::fuzz::{altered, Rng};
use crate::gen3::alive::parse as alive;
use crate::gen3::cmds::mcc_update_reply;
use crate::gen3::nvm::parse as nvm;
use crate::gen3::packet::parse as packet;
use crate::gen3::scan::complete;
use crate::gen3::station::queue::parse_queue_reply;
use crate::gen3::station::session::parse_notif;
use crate::gen3::tx_resp::parse as tx_resp;

use super::within;

const ROUNDS: u32 = 40_000;

#[test]
fn a_receive_buffer_yields_only_its_own_bytes() {
    let mut seed = vec![0u8; 256];
    seed[0..4].copy_from_slice(&(120u32).to_le_bytes());
    seed[4..8].copy_from_slice(&[0xC1, 0, 0x00, 0x80]);
    let whole = packet(&seed).map(|p| (p.cmd, p.group, p.sequence, p.payload.len()));
    let mut r = Rng::seeded(0x6E3);
    for round in 0..ROUNDS {
        let rb = if round % 3 == 0 { r.noise(300) } else { r.mutate(&seed) };
        let Some(p) = packet(&rb) else { continue };
        assert!(within(&rb, p.payload), "round {round}");
        if !altered(&seed, &rb) {
            let got = Some((p.cmd, p.group, p.sequence, p.payload.len()));
            assert_eq!(got, whole, "round {round}: bytes past the packet change nothing");
        }
    }
}

#[test]
fn notifications_and_replies_never_read_past_their_payload() {
    let mut r = Rng::seeded(0xA11E);
    for round in 0..ROUNDS {
        let len = [8, 16, 28, 68, 116, 128, 144, 256][round as usize % 8];
        let p = if round % 2 == 0 { r.noise(320) } else { r.mutate(&vec![0u8; len]) };
        for ver in [None, Some(3), Some(4), Some(5), Some(6), Some(7)] {
            let _ = alive(&p, ver);
        }
        let _ = (nvm(&p, false), nvm(&p, true), tx_resp(&p), parse_notif(&p), complete(&p));
        if let Some(q) = parse_queue_reply(&p) {
            assert_eq!(p.len(), 8, "round {round}: only the 8-byte reply");
            let _ = q;
        }
        for (ver, ax) in [(3, false), (4, true), (8, false)] {
            let _ = mcc_update_reply(&p, ver, ax);
        }
    }
}
