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

/*
 * Reading a response up to the blank line after its headers; what came
 * after it is the body's first bytes.
 */

use alloc::vec::Vec;

use nonos_libc::mk_uptime_ms;
use nonos_tls::stream::Stream;

use super::fault::Fault;
use crate::net::Link;

const HEAD_MAX: usize = 32 * 1024;
/* How long a mirror may take to answer, the Nym mixnet's round trips included. */
const ANSWER_MS: i64 = 90_000;

pub fn head(tls: &mut Stream, link: &mut Link) -> Result<(Vec<u8>, Vec<u8>), Fault> {
    let until = mk_uptime_ms().saturating_add(ANSWER_MS);
    let mut got = Vec::new();
    loop {
        if let Some(end) = got.windows(4).position(|w| w == b"\r\n\r\n") {
            let rest = got.split_off(end + 4);
            return Ok((got, rest));
        }
        if got.len() > HEAD_MAX {
            return Err(Fault::Unusable("the mirror's answer has no end to its headers"));
        }
        let more = tls.read(link).map_err(|_| Fault::Net("the connection to the mirror broke"))?;
        if more.is_empty() {
            if tls.is_done() || mk_uptime_ms() > until {
                return Err(Fault::Net("the mirror did not answer"));
            }
            crate::serve::idle(10);
        }
        got.extend_from_slice(&more);
    }
}
