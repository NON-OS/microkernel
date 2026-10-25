// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Key reinstallation through iwlwifi's receive path (Port::service), as
//! rtl8821ce's krack.rs holds it: no message 3 again or replayed, and no
//! group key held already, puts a key in the firmware twice (12.7.6.4).

use nonos_wifi_core::rsn::JoinPolicy;

use super::drive::run_join;
use super::krack_rig::{deliver, message3_again, SEC_KEY};
use crate::join_rig::*;

#[test]
fn a_message_3_sent_again_is_answered_and_installs_nothing() {
    let r = rig(Security::Wpa2, Switches::default());
    run_join(&r, JoinPolicy::ANY, |fw, out, _| {
        let mut j = out.expect("joined");
        let (keys, replies) =
            (r.command_payloads(SEC_KEY.0, SEC_KEY.1).len(), r.ap.borrow().eapol_replies.len());
        deliver(&r, fw, &mut j, &message3_again(&r, 3));
        assert_eq!(r.ap.borrow().eapol_replies.len(), replies + 1, "message 4 sent again");
        assert!(j.link.new_group_key.is_none() && j.link.stats.rekeys == 0, "no group key held");
        assert!(j.rekey(fw).is_none(), "nothing to install");
        assert_eq!(r.command_payloads(SEC_KEY.0, SEC_KEY.1).len(), keys, "no key command");
    });
}

#[test]
fn a_replayed_message_3_is_neither_answered_nor_installed() {
    let r = rig(Security::Wpa2, Switches::default());
    run_join(&r, JoinPolicy::ANY, |fw, out, _| {
        let mut j = out.expect("joined");
        let (keys, replies) =
            (r.command_payloads(SEC_KEY.0, SEC_KEY.1).len(), r.ap.borrow().eapol_replies.len());
        deliver(&r, fw, &mut j, &message3_again(&r, 2));
        assert_eq!(r.ap.borrow().eapol_replies.len(), replies, "an old counter is not answered");
        assert!(j.rekey(fw).is_none());
        assert_eq!(r.command_payloads(SEC_KEY.0, SEC_KEY.1).len(), keys);
    });
}
