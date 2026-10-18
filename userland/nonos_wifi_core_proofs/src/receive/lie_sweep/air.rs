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

//! The access point's side of the lie sweep: whole protected frames, each
//! as a chip hands it up said decrypted, and the lies a chip can tell about
//! frames that are otherwise whole.

use crate::ap_sim::GTK;
use crate::receive::ccmp_tests::tk;
use crate::receive::chip_tests::chip;
use crate::receive::frame::{from_ap, AP, STA};
use crate::receive::join::ip_body;
use crate::sweep::rng::Rng;
use nonos_wifi_core::rsn::TemporalKey;
use nonos_wifi_core::station::{Ccmp, LinkStation};

/// The group key's index and the Key RSC it was given.
pub const GTK_ID: u8 = 1;
pub const RSC: u64 = 9;

/// A station keyed `ccmp` (pairwise under `tk()`), with the GTK.
pub fn keyed(ccmp: Ccmp) -> LinkStation {
    let mut s = LinkStation::new(STA);
    s.associate(AP, ccmp);
    s.set_group_key(GTK_ID, TemporalKey::Ccmp128(GTK), RSC);
    s
}

/// A whole frame with a body drawn from `rng`: to us under the pairwise key,
/// or to every station under `key`.
pub fn frame(rng: &mut Rng, group: bool, key: &[u8; 16], pn: u64, kid: u8) -> Vec<u8> {
    let body = ip_body(&rng.span(1, 40));
    let dst = if group { [0xff; 6] } else { STA };
    chip(&from_ap(dst, &body, Some(key), pn, kid, true), &body)
}

/// One lie about an otherwise whole frame, and whether the station must
/// call it a replay (else undecryptable). `pn` and `gpn` are the last
/// values each counter took; `mic` says whether the station checks MICs.
pub fn lie(rng: &mut Rng, pn: u64, gpn: u64, mic: bool) -> (Vec<u8>, bool) {
    let (ahead, d) = (1 + rng.below(1000) as u64, rng.next());
    match rng.below(if mic { 6 } else { 4 }) {
        0 => (frame(rng, false, &tk(), pn - d % (pn + 1), 0), true),
        1 => (frame(rng, true, &GTK, gpn - d % 4, GTK_ID), true),
        2 => (frame(rng, false, &tk(), pn + ahead, 1 + (d % 3) as u8), false),
        3 => (frame(rng, true, &GTK, gpn + ahead, [0, 2, 3][(d % 3) as usize]), false),
        4 => (frame(rng, true, &tk(), gpn + ahead, GTK_ID), false),
        _ => {
            let group = rng.below(2) == 0;
            let (key, last, kid) = if group { (GTK, gpn, GTK_ID) } else { (tk(), pn, 0) };
            let mut f = frame(rng, group, &key, last + ahead, kid);
            let at = 34 + rng.below(f.len() - 34);
            f[at] ^= 1 << rng.below(8);
            (f, false)
        }
    }
}
