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

//! A lit transcoder's way to its monitor, read back without a write.

use crate::engine::pipe::Transcoder;
use crate::engine::regs::trans_ddi_func_ctl;
use crate::engine::route::{read, Link, Route};
use crate::model::Gen9;

fn with(t: Transcoder, ctl: u32) -> Gen9 {
    let mut m = Gen9::new(1);
    m.set(trans_ddi_func_ctl(t), ctl);
    m
}

#[test]
fn the_edp_transcoder_is_ddi_a_as_displayport() {
    let mut m = with(Transcoder::Edp, (1 << 31) | (4 << 12) | (3 << 1));
    let r = read(&mut m, Transcoder::Edp);
    assert_eq!(r, Some(Route { ddi: 0, link: Link::Dp { lanes: 4 } }));
}

#[test]
fn a_pipe_transcoder_names_its_ddi_and_protocol() {
    let mut m = with(Transcoder::B, (1 << 31) | (2 << 28) | (2 << 24) | (1 << 1));
    let dp = Some(Route { ddi: 2, link: Link::Dp { lanes: 2 } });
    assert_eq!(read(&mut m, Transcoder::B), dp, "DisplayPort on DDI C, two lanes");
    let mut m = with(Transcoder::A, (1 << 31) | (1 << 28));
    assert_eq!(read(&mut m, Transcoder::A), Some(Route { ddi: 1, link: Link::Hdmi }));
    let mut m = with(Transcoder::A, (1 << 31) | (4 << 28) | (1 << 24));
    assert_eq!(read(&mut m, Transcoder::A), Some(Route { ddi: 4, link: Link::Dvi }));
}

#[test]
fn an_off_function_an_mst_stream_or_no_ddi_is_no_route() {
    assert_eq!(read(&mut with(Transcoder::A, 1 << 28), Transcoder::A), None, "off");
    let mst = (1 << 31) | (1 << 28) | (3 << 24);
    assert_eq!(read(&mut with(Transcoder::A, mst), Transcoder::A), None, "MST");
    assert_eq!(read(&mut with(Transcoder::A, 1 << 31), Transcoder::A), None, "no DDI");
}
