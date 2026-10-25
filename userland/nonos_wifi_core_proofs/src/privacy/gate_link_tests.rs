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

//! Rows 1.10 and 1.11 on the air: a protected session whose AP's signal is
//! weak does not look while it runs an SA Query, which an unprotected
//! deauthentication in the AP's name starts.

use crate::pmf_sim::AP_RSNE_MFP;
use crate::roam::two_aps::{run_until, sent, two_aps_with, AA};
use crate::soft::frames::mgmt_from;
use crate::soft::rig::Station;

pub(super) fn protected() -> Station {
    let mfp = |b: &mut crate::soft::ap::SimAp| b.rsne = AP_RSNE_MFP.to_vec();
    let st = two_aps_with(mfp, mfp, [-85, -40]);
    assert!(st.session().unwrap().pmf, "a protected session");
    st
}

pub(super) fn clock(st: &Station) -> u64 {
    st.radio().unwrap().clock.get()
}

/// An unprotected deauthentication, which the station answers with an SA
/// Query it must see through at home.
pub(super) fn deauth(st: &mut Station) {
    let sta = crate::soft::join::sta_of(st);
    let air = st.radio_mut().unwrap();
    air.inbox.push_back(mgmt_from(AA, 12, sta, &7u16.to_le_bytes()));
}

#[test]
fn a_protected_session_heard_weak_looks() {
    let mut st = protected();
    assert!(run_until(&mut st, |s| !sent(s, 0x48).is_empty()), "it looked");
}

#[test]
fn no_look_starts_while_an_sa_query_runs() {
    let mut first = protected();
    assert!(run_until(&mut first, |s| !sent(s, 0x48).is_empty()));
    // The query starts shortly before the look would, and outlasts it.
    let due = clock(&first);
    let mut st = protected();
    assert!(run_until(&mut st, |s| clock(s) + 300 >= due));
    deauth(&mut st);
    assert!(!run_until(&mut st, |s| !sent(s, 0x48).is_empty()), "never away");
    assert!(!sent(&st, 0xd0).is_empty(), "it ran the SA Query");
}
