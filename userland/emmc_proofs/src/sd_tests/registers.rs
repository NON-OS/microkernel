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

//! The SD registers the driver decodes, held to the model card's.

use crate::emmc::sd::cmds::{if_cond_echoed, IF_COND_ARG};
use crate::emmc::sd::regs::{ccs, sd_cid, sd_sectors};
use crate::model::sd::{sdsc, SdCard, SdCfg};

#[test]
fn the_cid_names_the_card() {
    let cid = sd_cid(SdCard::new(SdCfg::default()).cid());
    assert_eq!((cid.mid, cid.oid, &cid.pnm), (0x03, b'S', b"SL32G "));
    assert_eq!((cid.prv, cid.psn, cid.mdt), (0x80, 0x1234_abcd, 0x18));
}

#[test]
fn the_csd_gives_the_size_by_its_version() {
    for cfg in [SdCfg::default(), sdsc()] {
        let card = SdCard::new(cfg);
        assert_eq!(sd_sectors(card.csd()), Some(card.capacity()));
    }
    let sduc = SdCard::new(SdCfg { csd_structure: 2, ..SdCfg::default() });
    assert_eq!(sd_sectors(sduc.csd()), None);
    let odd = SdCard::new(SdCfg { read_bl_len: 12, ..sdsc() });
    assert_eq!(sd_sectors(odd.csd()), None, "a block length past 2048");
}

#[test]
fn cmd8_and_the_capacity_bit() {
    assert!(if_cond_echoed(IF_COND_ARG) && if_cond_echoed(0x0300_01aa));
    assert!(!if_cond_echoed(0x2aa) && !if_cond_echoed(0x1ab));
    assert!(ccs(0xc0ff_8000) && !ccs(0x80ff_8000));
}
