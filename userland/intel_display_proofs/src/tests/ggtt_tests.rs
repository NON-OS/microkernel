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

//! GGTT entries, the flush after them, and where the driver's runs go.

use crate::engine::error::Error;
use crate::engine::ggtt::{map, unmap};
use crate::engine::ggtt_pte::Pte;
use crate::engine::ggtt_range::{entries, pick};
use crate::model::Gen9;

#[test]
fn an_entry_is_the_page_address_and_present() {
    assert_eq!(Pte::Wide.encode(0x12_3456_7000), 0x12_3456_7001);
    assert_eq!(Pte::Wide.encode(0x7F_FFFF_F000), 0x7F_FFFF_F001);
}

#[test]
fn map_writes_every_entry_then_flushes_once() {
    let mut m = Gen9::new(16);
    map(&mut m, Pte::Wide, 4, &[0x1000, 0x5000, 0x9000], 16).unwrap();
    assert_eq!(&m.ggtt[4..7], &[0x1001, 0x5001, 0x9001]);
    assert_eq!(m.flushes, 1);
    unmap(&mut m, Pte::Wide, 4, 2, 16).unwrap();
    assert_eq!(&m.ggtt[4..7], &[0, 0, 0x9001]);
    assert_eq!(m.flushes, 2);
}

#[test]
fn a_run_past_the_table_or_an_odd_page_is_refused_untouched() {
    let mut m = Gen9::new(16);
    assert_eq!(map(&mut m, Pte::Wide, 15, &[0x1000, 0x2000], 16), Err(Error::Argument));
    assert_eq!(map(&mut m, Pte::Wide, 0, &[0x1800], 16), Err(Error::Argument));
    let far = map(&mut m, Pte::Wide, 0, &[0x80_0000_0000], 16);
    assert_eq!(far, Err(Error::Argument), "past 39 bits");
    assert_eq!(unmap(&mut m, Pte::Wide, 10, 7, 16), Err(Error::Argument));
    assert_eq!(unmap(&mut m, Pte::Wide, 10, 0, 16), Err(Error::Argument));
    assert!(m.ggtt.iter().all(|&e| e == 0) && m.flushes == 0);
}

#[test]
fn the_table_size_comes_from_ggms() {
    assert_eq!(entries(Pte::Wide, 0), 0);
    assert_eq!(entries(Pte::Wide, 1 << 6), 262_144);
    assert_eq!(entries(Pte::Wide, 3 << 6), 1_048_576, "8 MiB of entries maps 4 GiB");
    assert_eq!(entries(Pte::Wide, 0xFF3F), 0, "other bits ignored");
}

#[test]
fn runs_come_from_the_top_clear_of_the_firmware_surface() {
    assert_eq!(pick(1024, 100, 64, (0, 500)), Some(896));
    assert_eq!(pick(1024, 100, 64, (900, 1024)), Some(768));
    assert_eq!(pick(1024, 600, 64, (300, 500)), None);
    assert_eq!(pick(1024, 200, 64, (100, 1024)), None);
    assert_eq!(pick(1024, 0, 64, (0, 0)), None);
    assert_eq!(pick(100, 101, 1, (0, 0)), None);
    for avoid in [(0, 10), (500, 600), (1000, 1024)] {
        let s = pick(1024, 64, 64, avoid).unwrap();
        assert!(s.is_multiple_of(64) && s + 64 <= 1024 && (s + 64 <= avoid.0 || s >= avoid.1));
    }
}
