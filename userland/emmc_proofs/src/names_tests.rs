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

//! The model and serial a part is named by.

use crate::emmc::info::*;
use crate::emmc::mmc::Cid;
use crate::model::{CardCfg, HostCfg, Rig};

#[test]
fn identify_names_the_maker_part_and_serial() {
    let cid =
        Cid { mid: 0x15, cbx: 1, oid: 0, pnm: *b"8GTF4R", prv: 0x02, psn: 0xdead_beef, mdt: 0 };
    let n = names(&cid);
    assert_eq!(n.model(), b"Samsung 8GTF4R");
    assert_eq!(&n.serial, b"DEADBEEF");
}

#[test]
fn an_unknown_maker_is_called_emmc_and_names_stay_printable() {
    let cid = Cid {
        mid: 0,
        cbx: 0,
        oid: 0,
        pnm: [b'A', 0, 0x7f, 0xff, b' ', b'z'],
        prv: 0,
        psn: 0x0000_00a1,
        mdt: 0,
    };
    let n = names(&cid);
    assert_eq!(n.model(), b"eMMC A??? z");
    assert_eq!(&n.serial, b"000000A1");
    let blank = Cid { pnm: *b"      ", ..cid };
    assert_eq!(names(&blank).model(), b"eMMC");
    for mid in 0..=255u8 {
        let n = names(&Cid { mid, pnm: [0xff; 6], ..cid });
        assert!(n.model_len <= MODEL_MAX);
        assert!(n.model().iter().all(|c| (0x20..0x7f).contains(c)));
    }
}

#[test]
fn the_part_served_is_named_on_the_console() {
    let rig = Rig::new(HostCfg::default(), CardCfg::default());
    let disk = rig.disk(true).expect("bring-up");
    disk.say_part();
    assert_eq!(disk.names().model(), b"SanDisk DF4032");
    assert_eq!(&disk.names().serial, b"12345678");
    assert!(
        rig.logged("serving SanDisk DF4032, serial 12345678, OCR 0xc0ff8080"),
        "{:#?}",
        rig.logs()
    );
}
