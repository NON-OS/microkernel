// NONOS Operating System (AGPL-3.0-or-later)
//! The control family's replies from a radio that is down or scanning.

use nonos_wifi_core::scan_list::{ScanResults, FLAG_WPA2};

use crate::control::{answer, Heard, View, REPLY_MAX};
use crate::gen3::bringup::BootError;
use crate::gen3::outcome::*;

pub(super) fn request(op: u16, rid: u32) -> Vec<u8> {
    let mut r = 0x5749_4649u32.to_le_bytes().to_vec();
    r.extend_from_slice(&op.to_le_bytes());
    r.extend_from_slice(&rid.to_le_bytes());
    r
}

pub(super) fn word(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}

#[test]
fn a_radio_that_is_down_reports_how_far_it_got_and_refuses_the_rest() {
    let f = Failure::Boot(BootError::NoAliveCause);
    let view = View { stage: f.stage(), step: f.step(), detail: f.detail(), heard: None };
    let mut out = vec![0u8; REPLY_MAX];
    let n = answer(&request(4, 77), &view, &mut out).unwrap();
    assert_eq!(n, 10 + 30);
    assert_eq!(&out[..10], &request(4, 77)[..], "the header is echoed");
    assert_eq!((out[10], out[11]), (STAGE_FIRMWARE_FAILED, STEP_BOOT));
    assert_eq!((word(&out, 12), word(&out, 16), word(&out, 20)), (0x30, 0, 0), "no card ids");
    for op in [1u16, 2, 3, 5, 99] {
        let n = answer(&request(op, 1), &view, &mut out).unwrap();
        assert_eq!((n, word(&out, 10) as i32), (14, -38), "op {op}");
    }
}

#[test]
fn a_scanning_radio_reports_ready_and_answers_a_scan_from_what_it_heard() {
    let mut list = ScanResults::new();
    list.heard([2, 0, 0, 0, 0, 1], b"Home", 1 | FLAG_WPA2, 80);
    let heard = || Some(Heard { list: &list, sweeps: 3, stalls: 1, frames: 40, beacons: 25 });
    let view = View { stage: STAGE_READY, step: 0, detail: 0, heard: heard() };
    let mut out = vec![0u8; REPLY_MAX];
    let n = answer(&request(4, 1), &view, &mut out).unwrap();
    assert_eq!(out[10], 0, "Ready");
    assert_eq!((word(&out, 24), word(&out, 28), word(&out, 32), word(&out, 36)), (3, 1, 40, 25));
    assert_eq!(n, 40);
    let n = answer(&request(3, 2), &view, &mut out).unwrap();
    assert_eq!((word(&out, 10), word(&out, 14), word(&out, 18)), (3, 40, 25), "sweeps, frames, beacons");
    assert_eq!(&out[22..n], &[1, 80, 1 | FLAG_WPA2, 4, b'H', b'o', b'm', b'e']);
    // A join that reaches `answer` (the radio cannot run one) is refused,
    // and says so at once.
    let n = answer(&request(1, 3), &view, &mut out).unwrap();
    assert_eq!((n, word(&out, 10) as i32), (14, -38));
}
