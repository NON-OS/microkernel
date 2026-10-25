// NONOS Operating System (AGPL-3.0-or-later)
//! What is not a control request is answered with nothing, and joins run
//! here.

use nonos_wifi_core::scan_list::ScanResults;

use crate::control::{answer, Heard, View, REPLY_MAX};
use crate::gen3::outcome::*;
use super::control::{request, word};

#[test]
fn what_is_not_a_control_request_is_answered_with_nothing() {
    use crate::control::{route, Route};
    let view = View { stage: 0, step: 0, detail: 0, heard: None };
    let mut out = vec![0u8; REPLY_MAX];
    let mut niwf = 0x4E49_5746u32.to_le_bytes().to_vec();
    niwf.extend_from_slice(&[0; 16]);
    assert_eq!(answer(&niwf, &view, &mut out), None, "the deleted protocol's tag");
    for can_join in [true, false] {
        assert_eq!(route(&niwf, can_join), Route::Other, "nothing serves that tag now");
        assert_eq!(route(&niwf[..3], can_join), Route::Other, "shorter than a tag");
        assert_eq!(route(&[], can_join), Route::Other, "empty");
    }
    assert_eq!(answer(&request(4, 1)[..9], &view, &mut out), None, "shorter than the header");
    let mut small = vec![0u8; REPLY_MAX - 1];
    assert_eq!(answer(&request(4, 1), &view, &mut small), None, "a reply buffer too small for a scan");
}

/// The Wi-Fi client sends a join only to a driver its table says joins. This
/// driver runs a join whenever its radio can (the request goes to the radio,
/// not to `answer`), so the table lists it as joining; a radio that cannot
/// still answers with the code the client gives a driver that does not join.
#[test]
fn the_client_sends_joins_to_this_driver_and_it_runs_them() {
    use crate::control::{route, Route};
    use crate::wifi_services::{CANNOT_JOIN, SERVICES};
    const OP_CONNECT: u16 = 1;
    let list = ScanResults::new();
    let mut join = request(OP_CONNECT, 9);
    join.extend_from_slice(&[4, b'h', b'o', b'm', b'e', 8]);
    join.extend_from_slice(b"passw0rd");
    join.push(0);
    assert_eq!(route(&join, true), Route::Join(OP_CONNECT), "a radio that can join runs it");
    for heard in [None, Some(Heard { list: &list, sweeps: 1, stalls: 0, frames: 0, beacons: 0 })] {
        let view = View { stage: STAGE_READY, step: 0, detail: 0, heard };
        assert_eq!(route(&join, false), Route::Control);
        let mut out = vec![0u8; REPLY_MAX];
        let n = answer(&join, &view, &mut out).expect("a control request is answered");
        assert_eq!(n, 10 + 4);
        assert_eq!(word(&out, 10) as i32, CANNOT_JOIN, "the join was answered as something else");
    }
    let me = SERVICES.iter().find(|s| s.name == b"driver.iwlwifi0").expect("listed");
    assert!(me.joins, "the client would answer for this driver instead of asking it");
    let realtek = SERVICES.iter().find(|s| s.name == b"driver.rtl8821ce0").expect("listed");
    assert!(realtek.joins);
    assert_eq!(SERVICES[0].name, b"driver.rtl8821ce0", "the RTL8821CE is still asked first");
}
