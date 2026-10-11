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

//! Which protocol a slot is tried with: a removable slot tries SD and then
//! MMC; an embedded slot, and a known Intel eMMC host, MMC alone, exactly
//! as before SD existed.

use crate::model::{sd_slot, CardCfg, HostCfg, Rig};

#[test]
fn an_mmc_card_in_a_removable_slot_is_found_after_sd_is_tried() {
    let rig = Rig::new(sd_slot(), CardCfg::default());
    let disk = rig.disk(false).expect("bring-up");
    assert!(!disk.card.sd);
    let i = rig.indices();
    // CMD8 and CMD55 go unanswered; then the MMC sequence from CMD0.
    assert_eq!(&i[..4], &[0, 8, 55, 0], "{i:?}");
    assert_eq!(i[4], 1);
    rig.assert_clean();
}

#[test]
fn an_embedded_slot_is_never_asked_for_an_sd_card() {
    for intel in [false, true] {
        let rig = Rig::new(HostCfg::default(), CardCfg::default());
        rig.disk(intel).expect("bring-up");
        let i = rig.indices();
        assert_eq!(&i[..2], &[0, 1], "intel {intel}");
        assert!(!i.contains(&55));
        rig.assert_clean();
    }
}

#[test]
fn a_removable_slot_without_three_volts_skips_sd() {
    // 1.8 V only, slot type 0.
    let host = HostCfg { caps: (200 << 8) | 1 << 19 | 1 << 21 | 1 << 26 | 1 << 28, ..sd_slot() };
    let rig = Rig::new(host, CardCfg::default());
    let disk = rig.disk(false).expect("bring-up");
    assert!(!disk.card.sd);
    assert!(rig.logged("slot has no 3.3 V supply"));
    assert!(!rig.indices().contains(&55));
    rig.assert_clean();
}
