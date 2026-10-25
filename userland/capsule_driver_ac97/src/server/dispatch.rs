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

//! One request. Only `OP_WRITE_PCM` carries a payload; any other op that
//! names one is refused. The ops driver.hda0 also serves for its own tools
//! (controller info, codec lists, a test tone) are refused here: the audio
//! server never sends them.

use super::respond::{output, status};
use super::stream::Stream;
use crate::protocol::{
    parse, refused, Request, E_AGAIN, E_INVAL, E_OK, HDR_LEN, MAX_PCM_CHUNK, OP_HEALTHCHECK,
    OP_OUTPUT_STATUS, OP_STREAM_START, OP_STREAM_STOP, OP_WRITE_PCM,
};
use crate::setup::Driver;

pub fn serve(d: &mut Driver, st: &mut Stream, sender: u32, msg: &[u8], tx: &mut [u8]) {
    let Some(req) = parse(msg) else {
        return status(sender, &refused(msg), E_INVAL, tx);
    };
    if req.payload_len != 0 && req.op != OP_WRITE_PCM {
        return status(sender, &req, E_INVAL, tx);
    }
    let errno = match req.op {
        OP_HEALTHCHECK => E_OK,
        OP_OUTPUT_STATUS => return output(sender, &req, &d.ident, tx),
        OP_STREAM_START if st.start(d) => E_OK,
        OP_STREAM_START => E_INVAL,
        OP_STREAM_STOP => {
            st.stop(d);
            E_OK
        }
        OP_WRITE_PCM => write_pcm(st, &req, msg),
        _ => E_INVAL,
    };
    status(sender, &req, errno, tx);
}

/// Queued whole or refused: `E_AGAIN` while the queue lacks room, and the
/// server sends the same block again.
fn write_pcm(st: &mut Stream, req: &Request, msg: &[u8]) -> i32 {
    let want = req.payload_len as usize;
    let avail = msg.len().saturating_sub(HDR_LEN);
    if want == 0 || want > MAX_PCM_CHUNK || want > avail {
        return E_INVAL;
    }
    if st.q.is_full_for(want) {
        return E_AGAIN;
    }
    st.q.push(&msg[HDR_LEN..HDR_LEN + want]);
    E_OK
}
