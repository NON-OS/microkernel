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

//! What the shell reads of INPUT_SOURCES is what the compositor writes,
//! and an answer that lies in its counts is cut to what it can hold; every
//! input has a name that fits, and every word the menu says fits a toast.

use crate::input_list::{encode, Offered, INPUT_SOURCES_LEN};
use crate::state::input_menu::{NONE_OFFERED, REFUSED, SWITCHED};
use crate::state::input_names::name;
use crate::state::input_offers::{read, ANSWER_LEN};
use crate::state::toast::TOAST_TEXT_MAX;

#[test]
fn the_shell_reads_what_the_compositor_writes() {
    assert_eq!(ANSWER_LEN, INPUT_SOURCES_LEN);
    let d = |i: u32| Offered {
        identity: 0x8000_0000 | i,
        now: 0x11,
        inputs: [i as u8 + 1; 8],
        n: 1 + i as u8,
    };
    for k in 0..=4u32 {
        let list: Vec<Offered> = (0..k).map(d).collect();
        let (offers, n) = read(&encode(&list));
        assert_eq!(n, (k as usize).min(3));
        for (o, w) in offers[..n].iter().zip(&list) {
            assert_eq!((o.identity, o.now, o.n, o.inputs), (w.identity, w.now, w.n, w.inputs));
        }
    }
}

#[test]
fn an_answer_that_lies_in_its_counts_is_cut() {
    let mut a = [0xFFu8; ANSWER_LEN];
    a[..4].copy_from_slice(&u32::MAX.to_le_bytes());
    let (offers, n) = read(&a);
    assert_eq!(n, 3);
    assert!(offers.iter().all(|o| o.n == 8), "n of 255 read as eight");
}

#[test]
fn every_input_is_named_within_its_room_and_every_notice_fits_a_toast() {
    let mut out = [0u8; 16];
    for v in 0..=255u8 {
        let k = name(v, &mut out);
        assert!(k > 0 && out[..k].is_ascii(), "{v:#x}");
    }
    let named = |v: u8| {
        let mut o = [0u8; 16];
        let k = name(v, &mut o);
        o[..k].to_vec()
    };
    assert_eq!((named(0x11), named(0x0F)), (b"HDMI 1".to_vec(), b"DisplayPort 1".to_vec()));
    assert_eq!(named(0xC4), b"Input 0xC4", "a monitor's own code");
    for said in [NONE_OFFERED, SWITCHED, REFUSED] {
        assert!(said.len() <= TOAST_TEXT_MAX && said.is_ascii());
    }
}
