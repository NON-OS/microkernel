// NONOS Operating System (AGPL-3.0-or-later)
//! A firmware that will not go on channel, answers a queue badly or
//! raises its error cause ends the join, taking down what was up.

use nonos_wifi_core::rsn::JoinPolicy;

use super::drive::run_join;
use super::refusals::after_setup;
use super::rig::{LINK, PHY, QUEUE, RLC, STA_CFG, STA_RM, TEARDOWN_FROM_SESSION};
use crate::gen3::join::bss::SetupError;
use crate::gen3::join::run::JoinEnd;
use crate::gen3::station::session::Session;
use crate::join_rig::*;

#[test]
fn a_firmware_that_will_not_put_the_radio_on_channel_ends_the_join_before_any_frame() {
    for (reply, want) in [
        (SessionReply::Refuse, JoinEnd::Setup(SetupError::NoSession(Some(Session::Refused)))),
        (SessionReply::Never, JoinEnd::Setup(SetupError::NoSession(None))),
    ] {
        let r = rig_with(Security::Wpa2, Switches::default(), reply);
        let mut end = None;
        run_join(&r, JoinPolicy::ANY, |_, out, _| end = out.err());
        assert_eq!(end, Some(want));
        assert!(r.sent().is_empty(), "nothing was sent");
        assert_eq!(after_setup(&r), TEARDOWN_FROM_SESSION[1..].to_vec(), "no session to cancel");
    }
}

#[test]
fn a_bad_queue_reply_or_a_firmware_error_takes_down_what_was_up() {
    let r = rig(Security::Wpa2, Switches::default());
    r.model.s.borrow_mut().txq.as_mut().unwrap().queue_reply_len = 12;
    let mut end = None;
    run_join(&r, JoinPolicy::ANY, |_, out, _| end = out.err());
    assert_eq!(end, Some(JoinEnd::Setup(SetupError::BadQueueReply)));
    assert_eq!(r.commands(), vec![PHY, RLC, LINK, LINK, STA_CFG, QUEUE, STA_RM, LINK, PHY]);

    let r = rig(Security::Wpa2, Switches::default());
    // The firmware asserts on the join's first command.
    r.model.s.borrow_mut().fw_error_after = Some(0);
    let mut end = None;
    run_join(&r, JoinPolicy::ANY, |_, out, _| end = out.err());
    match end {
        Some(JoinEnd::Setup(SetupError::Command(c))) => assert_eq!((c.group, c.cmd), (0x1, 0x08)),
        other => panic!("{other:?}"),
    }
}
