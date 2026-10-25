// NONOS Operating System (AGPL-3.0-or-later)
//! Refusals end the join with their codes, and take down what was up.

use nonos_wifi_core::mlme::MlmeFailure;
use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::sae::SaeFailure;

use super::drive::run_join;
use super::rig::{LINK, MAC, POWER, SESSION, STA_CFG, TEARDOWN_FROM_SESSION};
use crate::gen3::join::run::JoinEnd;
use crate::join_rig::*;

pub(super) fn refused(sec: Security, sw: Switches) -> (Rig, JoinEnd) {
    let r = rig(sec, sw);
    let mut end = None;
    run_join(&r, JoinPolicy::ANY, |_, out, _| end = out.err());
    let e = end.expect("the join ended");
    (r, e)
}

// The commands after the join's setup, which must be the teardown.
pub(super) fn after_setup(r: &Rig) -> Vec<(u8, u8)> {
    r.commands()[8..].to_vec()
}

#[test]
fn refusals_end_the_join_with_their_codes_and_take_the_contexts_down() {
    let (r, e) = refused(Security::Wpa2, Switches { refuse_auth: Some(17), ..Switches::default() });
    assert_eq!(e, JoinEnd::Refused(MlmeFailure::AuthRejected(17)));
    assert_eq!(after_setup(&r), TEARDOWN_FROM_SESSION.to_vec());

    let (r, e) =
        refused(Security::Wpa2, Switches { refuse_assoc: Some(18), ..Switches::default() });
    assert_eq!(e, JoinEnd::Refused(MlmeFailure::AssocRejected(18)));
    assert_eq!(after_setup(&r), TEARDOWN_FROM_SESSION.to_vec());

    let (r, e) =
        refused(Security::Wpa2, Switches { deauth_instead_of_m3: true, ..Switches::default() });
    assert_eq!(e, JoinEnd::Refused(MlmeFailure::Left(15)));
    // Associated by then: the MAC context is marked unassociated again, and
    // the station's power command follows it either way.
    let mut want = vec![MAC, STA_CFG, LINK, POWER, SESSION, MAC, POWER];
    want.extend_from_slice(&TEARDOWN_FROM_SESSION[1..]);
    assert_eq!(after_setup(&r), want);
    assert!(r.command_payloads(0x5, 0x18).is_empty(), "no key was installed");

    let (r, e) = refused(
        Security::Sae { h2e: true },
        Switches { wrong_password: true, ..Switches::default() },
    );
    assert_eq!(e, JoinEnd::Refused(MlmeFailure::Sae(SaeFailure::BadConfirm)));
    assert_eq!(after_setup(&r), TEARDOWN_FROM_SESSION.to_vec());
}
