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

//! Segment pointer (E-DDC) lies: refused, acknowledged and ignored; and
//! the honest pointer, which reaches the third and fourth blocks.

use super::kit::{honest, read};
use super::sink::{Eddc, Sink};
use crate::model::Ticks;

#[test]
fn a_sink_without_e_ddc_naks_the_pointer_and_gives_its_first_segment() {
    let t = Ticks::new();
    let mut s = Sink::new(&t, honest(4));
    s.eddc = Eddc::Nak;
    assert_eq!(read(&mut s, &t).0, 256);
    let at_two = &s.log[2..];
    assert_eq!(at_two.len(), 3);
    assert!(at_two.iter().all(|x| x.segment == Some((1, false)) && x.read.is_none()));
}

#[test]
fn a_pointer_acknowledged_then_ignored_repeats_segment_zero_and_is_refused() {
    let t = Ticks::new();
    let mut s = Sink::new(&t, honest(4));
    s.eddc = Eddc::Ignore;
    let (n, out, _) = read(&mut s, &t);
    assert_eq!((n, &out[..256]), (256, &honest(4)[..256]));
}

#[test]
fn an_honest_sink_with_e_ddc_gives_all_four_blocks() {
    let t = Ticks::new();
    let mut s = Sink::new(&t, honest(4));
    let (n, out, _) = read(&mut s, &t);
    assert_eq!((n, &out[..]), (512, &honest(4)[..]));
    assert_eq!(s.log[3].segment, Some((1, true)));
}
