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

//! Each transport's latest route report, with its age on this board's clock.
//! Reports name no relay, gateway or key, so any caller may read them.

use nonos_route_proof::{Board, ANSWER_LEN};

use crate::protocol::{Request, E_INVAL, HDR_LEN, STATUS_LEN};
use crate::server::respond;

pub fn run(out: &mut [u8], req: &Request, board: &Board) -> usize {
    let dst = HDR_LEN + STATUS_LEN;
    if dst + ANSWER_LEN > out.len() {
        return respond::status(out, req, E_INVAL);
    }
    let now = nonos_libc::mk_time_millis().max(0) as u64;
    out[dst..dst + ANSWER_LEN].copy_from_slice(&board.answer(now));
    respond::with_payload(out, req, 0, ANSWER_LEN)
}
