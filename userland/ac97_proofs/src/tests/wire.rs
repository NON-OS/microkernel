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

//! The audio server's own sink wire (`capsule_audio/src/sink/wire.rs`)
//! against this driver: what the server builds is served, and what the
//! driver answers reads back as the server expects.

use super::util::{ask, up};
use crate::server::Stream;
use crate::sink_wire;

#[test]
fn the_server_s_writes_and_stream_control_are_served() {
    let (mut d, ich) = up();
    let mut st = Stream::default();
    let mut tx = [0u8; sink_wire::HDR_LEN];
    let n = sink_wire::start_request(5, &mut tx);
    assert!(sink_wire::reply_ok(&ask(&mut d, &mut st, &tx[..n])));
    assert!(st.running && ich.borrow().open());
    let pcm = vec![9u8; sink_wire::MAX_PCM_CHUNK];
    let mut out = vec![0u8; sink_wire::HDR_LEN + pcm.len()];
    let n = sink_wire::request(6, &pcm, &mut out);
    assert_eq!(sink_wire::reply_status(&ask(&mut d, &mut st, &out[..n])), 0);
    let n = sink_wire::stop_request(7, &mut tx);
    assert!(sink_wire::reply_ok(&ask(&mut d, &mut st, &tx[..n])));
    assert!(!st.running);
}

// Ready, with the line out flag `nonos_audio_proto::output` names.
#[test]
fn the_output_status_reads_as_ready_on_the_line_out() {
    let (mut d, _ich) = up();
    let mut tx = [0u8; sink_wire::HDR_LEN];
    let n = sink_wire::output_status_request(8, &mut tx);
    let r = ask(&mut d, &mut Stream::default(), &tx[..n]);
    assert_eq!(r.len(), sink_wire::OUTPUT_REPLY_LEN);
    assert_eq!(sink_wire::read_output_status(&r), Some((0, 1 << 2)));
    assert_eq!(&r[28..32], &[0x84, 0x83, 0x00, 0x76], "the codec's vendor id halves");
}
