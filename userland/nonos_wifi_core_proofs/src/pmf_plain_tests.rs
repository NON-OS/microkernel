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

/// With protection, a deauthentication the chip says it decrypted ends the
/// link only if its plaintext seals again to the MIC that came; a forged one
/// is refused, leaves the link, and moves no counter.
#[test]
fn a_chips_word_on_a_protected_deauthentication_is_checked() {
    use crate::pmf_frames::sealed;
    use crate::pmf_sim::joined_pmf;
    let (mut s, ptk) = joined_pmf();
    let honest = sealed(&mgmt(DEAUTH, SPA, &[3, 0]), 9, &ptk, true);
    let mut lie = honest.clone();
    lie[32] = 1;
    assert_eq!(s.receive_mgmt(&lie, true), MgmtRx::Dropped(MgmtDrop::Undecryptable));
    assert!(s.is_associated(), "the link stays");
    let q = sealed(&mgmt(13, SPA, &[8, 0, 5, 6]), 2, &ptk, true);
    assert!(matches!(s.receive_mgmt(&q, true), MgmtRx::Reply(_)), "no counter moved");
    assert_eq!(s.receive_mgmt(&honest, true), MgmtRx::Left(3));
}

/// The key index is outside the MIC, so a frame whole in every other octet
/// can name another: a unicast management frame names index 0, or it is
/// refused, whoever decrypted it.
#[test]
fn a_protected_deauthentication_naming_another_key_index_is_refused() {
    use crate::pmf_frames::sealed;
    use crate::pmf_sim::joined_pmf;
    let (mut s, ptk) = joined_pmf();
    for hw in [true, false] {
        let mut f = sealed(&mgmt(DEAUTH, SPA, &[3, 0]), 9, &ptk, hw);
        f[27] |= 1 << 6;
        assert_eq!(s.receive_mgmt(&f, hw), MgmtRx::Dropped(MgmtDrop::Undecryptable));
        assert!(s.is_associated());
    }
}
