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

//! A controller whose codec, link or engine does not come up is given back
//! whole: the DMA grant, both windows and the claim.

use nonos_libc::calls;

use super::util::up_with;
use crate::model::Options;

fn given_back_whole(opt: Options) {
    let (d, _ich) = up_with(opt);
    assert!(d.is_err(), "the bring-up must refuse");
    let got = calls();
    let tail = [("dma_unmap", 70), ("pio_release", 51), ("pio_release", 50), ("device_release", 3)];
    assert_eq!(got[got.len() - 4..], tail);
}

#[test]
fn no_codec_on_the_link_is_refused() {
    given_back_whole(Options { codec: false, ..Options::default() });
}

#[test]
fn a_codec_whose_dac_never_powers_up_is_refused() {
    given_back_whole(Options { powered: false, ..Options::default() });
}

// Reads float to all ones, ready bits and all; nothing must pass for a codec.
#[test]
fn windows_the_bridge_does_not_decode_are_refused() {
    given_back_whole(Options { decode_off: true, ..Options::default() });
}

#[test]
fn a_box_that_never_finishes_its_reset_is_refused() {
    given_back_whole(Options { rr_sticks: true, ..Options::default() });
}
