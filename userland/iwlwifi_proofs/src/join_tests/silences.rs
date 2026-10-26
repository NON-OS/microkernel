// NONOS Operating System (AGPL-3.0-or-later)
//! Silences end the join with their codes, the unanswered frame resent
//! first: at each silence until the idle tries or the step's own time
//! (`Mlme::deadline`) run out, whichever comes first.

use super::rig::TEARDOWN_FROM_SESSION;
use crate::gen3::join::exchange::limits::{IDLE_TRIES, RETX_MS};
use crate::gen3::join::run::JoinEnd;
use crate::join_rig::*;
use nonos_wifi_core::mlme::{ASSOC_MS, AUTH_MS, FOURWAY_MS};

use super::refusals::{after_setup, refused};

// The frames sent in a step of `ms`: the first, then one at each silence.
fn within(ms: u64) -> u32 {
    1 + IDLE_TRIES.min((ms / u64::from(RETX_MS)) as u32)
}

#[test]
fn silences_time_out_with_the_unanswered_frame_resent() {
    let (r, e) = refused(Security::Wpa2, Switches { silent: true, ..Switches::default() });
    assert_eq!(e, JoinEnd::TimedOut);
    let auths = r.sent().iter().filter(|s| s.frame[0] == 0xB0).count();
    assert_eq!(auths as u32, within(AUTH_MS), "sent, then resent at each silence");
    assert_eq!(after_setup(&r), TEARDOWN_FROM_SESSION.to_vec());

    let (r, e) =
        refused(Security::Wpa2, Switches { silent_after_auth: true, ..Switches::default() });
    assert_eq!(e, JoinEnd::TimedOut);
    assert_eq!(r.sent().iter().filter(|s| s.frame[0] == 0x00).count() as u32, within(ASSOC_MS));

    let (r, e) = refused(Security::Wpa2, Switches { silent_after_m1: true, ..Switches::default() });
    assert_eq!(e, JoinEnd::TimedOut);
    assert_eq!(
        r.sent().iter().filter(|s| s.frame[0] == 0x08).count() as u32,
        within(FOURWAY_MS),
        "message 2"
    );

    // A wrong WPA2 passphrase: message 3 fails its MIC at the station and is
    // dropped; the join times out with no key installed.
    let (r, e) = refused(Security::Wpa2, Switches { wrong_pmk: true, ..Switches::default() });
    assert_eq!(e, JoinEnd::TimedOut);
    assert!(r.command_payloads(0x5, 0x18).is_empty());
    assert!(!r.ap.borrow().keyed);
}
