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

//! The queries the loop dispatches, answered by the shipping handlers from
//! the buffer the loop replies from, for a driver that plays the ALC236.

use super::serving::{answer, ask, playing, tx};
use crate::controller::MAX_CODECS;
use crate::proofs::lifecycle::board::GCAP_4X4;
use crate::protocol::{
    E_OK, OP_CODEC_LIST, OP_CODEC_MASK, OP_CONTROLLER_INFO, OP_HEALTHCHECK, OP_OUTPUT_STATUS,
    OP_STREAM_LAYOUT, OUT_HEADPHONE, OUT_SPEAKER,
};
use crate::server::handlers::{
    codec_list, codec_mask, controller_info, health, output_status, stream_layout,
};

#[test]
fn every_query_is_answered_whole_with_what_the_bring_up_found() {
    let p = playing();
    let mut tx = tx();
    health::handle(&ask(OP_HEALTHCHECK, 0), &mut tx);
    assert_eq!(answer(OP_HEALTHCHECK), (E_OK, vec![]));
    controller_info::handle(&p.d, &ask(OP_CONTROLLER_INFO, 0), &mut tx);
    assert_eq!(answer(OP_CONTROLLER_INFO).1[..2], GCAP_4X4.to_le_bytes());
    codec_mask::handle(&p.d, &ask(OP_CODEC_MASK, 0), &mut tx);
    assert_eq!(answer(OP_CODEC_MASK), (E_OK, vec![1, 0, 0, 0, 1, 0, 0, 0]));
    stream_layout::handle(&p.d, &ask(OP_STREAM_LAYOUT, 0), &mut tx);
    assert_eq!(answer(OP_STREAM_LAYOUT).1[..4], 8u32.to_le_bytes(), "four in, four out");
    codec_list::handle(&p.d, &ask(OP_CODEC_LIST, 0), &mut tx);
    let (_, list) = answer(OP_CODEC_LIST);
    assert_eq!(list, [1, 0, 0, 0, 0, 1, 0xec, 0x10, 0x36, 0x02, 0, 0], "one ALC236 at 0");
    output_status::handle(&p.d.status, &ask(OP_OUTPUT_STATUS, 0), &mut tx);
    let (status, out) = answer(OP_OUTPUT_STATUS);
    assert_eq!((status, &out[..4], out[8]), (E_OK, &[0u8; 4][..], OUT_SPEAKER | OUT_HEADPHONE));
}

#[test]
fn a_link_with_every_codec_present_is_listed_in_the_buffer_the_loop_has() {
    let mut p = playing();
    p.d.codecs.iter_mut().for_each(|c| c.present = 1);
    codec_list::handle(&p.d, &ask(OP_CODEC_LIST, 0), &mut tx());
    let (_, list) = answer(OP_CODEC_LIST);
    assert_eq!(list[..4], (MAX_CODECS as u32).to_le_bytes());
    assert_eq!(list.len(), 4 + MAX_CODECS * 8);
}
