// NONOS Operating System (AGPL-3.0-or-later)
//! Running a join on the rig, and reading what it left behind.

use nonos_wifi_core::netif::wire;
use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::wpa::akm::Akm;

use crate::gen3::join::exchange::Progress;
use crate::gen3::join::run::{join, JoinEnd, Joined};
use crate::gen3::tx_cmd::{IWL_TX_FLAGS_CMD_RATE, IWL_TX_FLAGS_ENCRYPT_DIS};
use crate::join_rig::*;

pub(super) fn run_join(r: &Rig, policy: JoinPolicy, then: impl FnOnce(&mut crate::gen3::join::fw::Fw<'_, '_, crate::gen3_model::Model, crate::gen3_model::Mem, crate::gen3_model::Polls>, Result<Joined, JoinEnd>, Progress)) {
    let beacon = r.ap.borrow().beacon();
    let mut st = Station::new();
    with_fw(r, &mut st, |fw| {
        let mut p = Progress::default();
        let out = join(fw, &request(policy), &beacon, 6, 0x3, 0x3, &mut p);
        then(fw, out, p);
    });
}

pub(super) fn joined(r: &Rig) -> (Akm, Progress) {
    let mut got = None;
    run_join(r, JoinPolicy::ANY, |_, out, p| {
        let j = out.unwrap_or_else(|e| panic!("joined, not {e:?}"));
        assert!(j.link.station.is_associated(), "the port is open");
        got = Some((j.akm, p));
    });
    got.unwrap()
}

// Every frame the station queued until the port opened was authentication or
// association on the management queue, or EAPOL in the clear on the data
// queue, all at the lowest basic rate with no firmware key.
pub(super) fn only_handshake_frames_left(r: &Rig) {
    let sent = r.sent();
    assert!(!sent.is_empty());
    let (mgmt_q, data_q) = (sent[0].queue, sent.iter().find(|s| s.frame[0] == 0x08).map(|s| s.queue));
    for s in &sent {
        match s.frame[0] {
            0xB0 | 0x00 => assert_eq!(s.queue, mgmt_q, "management frames on the management queue"),
            0x08 => {
                assert!(is_clear_eapol(&s.frame), "a data frame before the port opened that is not EAPOL");
                assert_eq!(Some(s.queue), data_q);
            }
            other => panic!("frame {other:#x} left before the port opened"),
        }
        assert_eq!(s.flags, IWL_TX_FLAGS_CMD_RATE | IWL_TX_FLAGS_ENCRYPT_DIS);
        assert_eq!(s.rate, 0x4000, "1 Mb/s on antenna A, the lowest basic rate");
    }
    assert_ne!(Some(mgmt_q), data_q);
    assert_eq!(r.bc_mismatch(), 0, "every byte count entry matched its frame");
}

pub(super) fn netif(op: u16, rid: u32, payload: &[u8]) -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(&wire::MAGIC_NNET.to_le_bytes());
    v.extend_from_slice(&wire::VERSION.to_le_bytes());
    v.extend_from_slice(&op.to_le_bytes());
    v.extend_from_slice(&[0; 4]);
    v.extend_from_slice(&rid.to_le_bytes());
    v.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    v.extend_from_slice(payload);
    v
}

pub(super) fn status(out: &[u8]) -> i32 {
    i32::from_le_bytes(out[20..24].try_into().unwrap())
}
