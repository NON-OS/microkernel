// NONOS Operating System (AGPL-3.0-or-later)
//! A queue number the firmware gives is used only if the device can have it
//! (below 512, as `IWL_MAX_TVQM_QUEUES`), it is not the command queue, and
//! the other TID's queue does not already hold it. Otherwise the join ends
//! before any frame, nothing is sent on that queue, and the queue the
//! firmware holds is removed by its TID.

use nonos_wifi_core::rsn::JoinPolicy;

use super::drive::run_join;
use super::rig::{FLUSH, LINK, PHY, QUEUE, RLC, STA_CFG, STA_RM};
use crate::gen3::join::bss::queues::usable;
use crate::gen3::join::bss::SetupError;
use crate::gen3::join::run::JoinEnd;
use crate::join_rig::*;

#[test]
fn only_a_queue_the_device_can_have_and_nobody_holds_is_usable() {
    assert!(usable(1, None) && usable(511, Some(1)));
    assert!(!usable(0, None), "the command queue");
    assert!(!usable(512, None) && !usable(u16::MAX, None), "past the device's queues");
    assert!(!usable(7, Some(7)), "the other TID's queue");
}

#[test]
fn a_queue_number_the_device_cannot_have_ends_the_join_and_its_tid_is_removed() {
    for given in [0u16, 512, 0xFFFE] {
        let r = rig(Security::Wpa2, Switches::default());
        r.model.s.borrow_mut().txq.as_mut().unwrap().next_qid = given;
        let mut end = None;
        run_join(&r, JoinPolicy::ANY, |_, out, _| end = out.err());
        assert_eq!(end, Some(JoinEnd::Setup(SetupError::BadQueue(given))), "queue {given}");
        assert!(r.sent().is_empty(), "nothing was sent");
        let q = r.model.s.borrow();
        assert!(q.txq.as_ref().unwrap().sent.is_empty(), "no frame on queue {given}");
        drop(q);
        let up = [PHY, RLC, LINK, LINK, STA_CFG, QUEUE];
        let down = [FLUSH, QUEUE, STA_RM, LINK, PHY];
        assert_eq!(r.commands(), [&up[..], &down].concat(), "the queue removed by its TID");
    }
}
