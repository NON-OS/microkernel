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

//! One request, checked and handed to its handler, and what a stream's
//! start, stop or tone changes in the state the loop keeps between them.

use super::stream::Stream;
use crate::protocol::{
    decode_request, refused, E_INVAL, OP_CODEC_LIST, OP_CODEC_MASK, OP_CONTROLLER_INFO,
    OP_HEALTHCHECK, OP_OUTPUT_STATUS, OP_PLAY_TONE, OP_STREAM_LAYOUT, OP_STREAM_START,
    OP_STREAM_STOP, OP_WRITE_PCM,
};
use crate::server::{error, handlers};
use crate::setup::Driver;

pub(super) fn serve(driver: &mut Driver, msg: &[u8], tx: &mut [u8], st: &mut Stream) {
    let req = match decode_request(msg) {
        Some(r) => r,
        None => {
            error::reply_with_status(tx, &refused(msg), E_INVAL);
            return;
        }
    };
    if req.payload_len != 0 && req.op != OP_WRITE_PCM {
        error::reply_with_status(tx, &req, E_INVAL);
        return;
    }
    match req.op {
        OP_HEALTHCHECK => handlers::health::handle(&req, tx),
        OP_CONTROLLER_INFO => handlers::controller_info::handle(driver, &req, tx),
        OP_CODEC_MASK => handlers::codec_mask::handle(driver, &req, tx),
        OP_STREAM_LAYOUT => handlers::stream_layout::handle(driver, &req, tx),
        OP_CODEC_LIST => handlers::codec_list::handle(driver, &req, tx),
        OP_OUTPUT_STATUS => handlers::output_status::handle(&driver.status, &req, tx),
        OP_PLAY_TONE => {
            let was = st.running;
            handlers::play_tone::handle(driver, &req, tx, &mut st.played, &mut st.running);
            if st.running && !was {
                st.q.clear();
                st.rf.reset();
            }
        }
        OP_STREAM_START => {
            let was = st.running;
            handlers::stream::handle_stream_start(driver, &req, tx, &mut st.running);
            if st.running && !was {
                st.rf.reset();
            }
        }
        OP_STREAM_STOP => {
            let was = st.running;
            handlers::stream::handle_stream_stop(driver, &req, tx, &mut st.q, &mut st.running);
            if was && !st.running {
                st.rf.report();
            }
        }
        OP_WRITE_PCM => handlers::write_pcm::handle(&mut st.q, &req, msg, tx),
        _ => error::reply_with_status(tx, &req, E_INVAL),
    }
}
