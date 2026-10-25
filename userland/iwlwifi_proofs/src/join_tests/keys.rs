// NONOS Operating System (AGPL-3.0-or-later)
//! A key the firmware does not take, and a transmit response that is never
//! taken for a command's reply.

use nonos_wifi_core::rsn::JoinPolicy;

use super::drive::run_join;
use super::rig::KEY;
use crate::gen3::join::keys::KeyError;
use crate::gen3::join::run::JoinEnd;
use crate::gen3_model::{reply, Out, Seen};
use crate::join_rig::*;

#[test]
fn a_key_the_firmware_does_not_take_ends_the_join_and_removes_nothing_it_did_not_install() {
    let r = rig(Security::Wpa2, Switches::default());
    r.model.s.borrow_mut().responder =
        Some(Box::new(move |c: &Seen| match (c.group, c.cmd, c.payload.get(4)) {
            (0x5, 0x18, _) => vec![],
            (0x3, 0x05, Some(1)) => vec![reply(c, vec![]), session_notif(1, 1)],
            _ => vec![reply(c, vec![])],
        }));
    let mut end = None;
    run_join(&r, JoinPolicy::ANY, |_, out, _| end = out.err());
    assert!(matches!(end, Some(JoinEnd::Keys(KeyError::Pairwise(_)))), "{end:?}");
    let cmds = r.commands();
    assert_eq!(cmds.iter().filter(|c| **c == KEY).count(), 1, "only the add that went unanswered");
    // Its buffer may still be read, so the teardown sends nothing after it:
    // the serving loop restarts the firmware instead, which drops it all.
    assert_eq!(cmds.last(), Some(&KEY), "nothing sent into a firmware that stopped answering");
}

#[test]
fn a_transmit_response_is_never_taken_for_a_commands_reply() {
    let r = rig(Security::Wpa2, Switches::default());
    // The firmware answers with a TX response carrying the command's own
    // sequence first, then the real reply.
    r.model.s.borrow_mut().responder = Some(Box::new(|c: &Seen| {
        let mut resp = vec![0u8; 48];
        resp[0] = 1;
        vec![Out { cmd: 0x1C, group: 0, seq: c.seq, payload: resp }, reply(c, vec![7, 7])]
    }));
    let mut st = Station::new();
    with_fw(&r, &mut st, |fw| {
        assert_eq!(fw.command(0x3, 0x0C, &[0; 4]), Ok(vec![7, 7]));
    });
}
