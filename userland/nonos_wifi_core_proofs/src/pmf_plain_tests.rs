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

//! A link without management frame protection: no SA Query, and the AP's
//! plain deauthentication ends it as it always has, once it is the AP's.

use crate::ap_sim::{AA, SPA};
use crate::pmf_frames::mgmt;
use nonos_wifi_core::station::{Ccmp, LinkStation, MgmtDrop, MgmtRx};

const DEAUTH: u8 = 12;

#[test]
fn without_protection_a_plain_deauthentication_ends_the_link() {
    let mut s = LinkStation::new(SPA);
    s.associate(AA, Ccmp::Software { tk: [7; 16] });
    assert!(s.sa_query_start(0, [1, 1]).is_none(), "no protection, no query");
    let mut elsewhere = mgmt(DEAUTH, SPA, &[2, 0]);
    elsewhere[10] ^= 1;
    assert_eq!(s.receive_mgmt(&elsewhere, false), MgmtRx::Dropped(MgmtDrop::NotOurs));
    assert_eq!(
        s.receive_mgmt(&mgmt(8, SPA, &[0; 12]), false),
        MgmtRx::Dropped(MgmtDrop::NotHandled)
    );
    assert_eq!(s.receive_mgmt(&mgmt(DEAUTH, SPA, &[2, 0]), false), MgmtRx::Left(2));
    assert!(!s.is_associated());
}
