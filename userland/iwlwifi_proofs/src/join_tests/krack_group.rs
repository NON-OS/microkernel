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

//! A group key given again, then the join's own handed back, reaches the
//! firmware once: the station keeps a key it holds (802.11-2020 12.7.7.3).

use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::wpa::akm::Akm;

use super::drive::run_join;
use super::krack_rig::{deliver, SEC_KEY};
use crate::ap_sim::{group1, gtk_kde, GTK};
use crate::join_rig::*;

#[test]
fn a_group_key_given_again_or_given_back_reaches_the_firmware_once() {
    let r = rig(Security::Wpa2, Switches::default());
    run_join(&r, JoinPolicy::ANY, |fw, out, _| {
        let mut j = out.expect("joined");
        let keys = r.command_payloads(SEC_KEY.0, SEC_KEY.1).len();
        let (next, ptk) = ([0x77u8; 16], r.ap.borrow().ptk);
        for (replay, id, gtk) in [(3, 2, &next[..]), (4, 2, &next[..]), (5, 1, &GTK[..])] {
            deliver(&r, fw, &mut j, &group1(Akm::Psk, replay, &ptk, &gtk_kde(id, gtk)));
            let _ = j.rekey(fw);
        }
        assert_eq!(j.link.stats.rekeys, 1, "one new key, not three");
        assert_eq!(j.keys.group, Some(2));
        let added: Vec<_> = r.command_payloads(SEC_KEY.0, SEC_KEY.1)[keys..]
            .iter()
            .filter(|p| p[0] == 1)
            .cloned()
            .collect();
        assert_eq!(added.len(), 1, "one install in the firmware");
        assert_eq!((added[0][8], &added[0][16..32]), (2, &next[..]));
    });
}
