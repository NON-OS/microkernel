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

use nonos_libc::mk_debug;
use nonos_policy_proto::Field;

use crate::protocol::{E_BAD_LEN, E_NAME_REFUSED, E_NO_HANDLE};
use crate::server::parse_req::Request;
use crate::sockets::{Kind, SocketKey, SOCKETS};
use crate::{clients::dns, state};

use super::host_target::{host_target, is_direct};
use super::{finish_host, parse_host, status_host};

/* Said once per refusal, without the name, which the log does not keep. */
const NAME_REFUSED: &[u8] = b"[SOCKETS] connect by name refused: the socket is a mixnet one \
or the chosen network is not Direct, and the name would be resolved in the clear\n";

/* Whether the person's chosen network is Direct, asked afresh for each
 * lookup (`is_direct` holds the rule). */
fn direct_chosen() -> bool {
    is_direct(
        nonos_policy_client::lookup()
            .and_then(|port| nonos_policy_client::get_u8(port, Field::NetworkRoute)),
    )
}

pub fn handle_host(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let (handle, port, host) = match parse_host::parse(body) {
        Some(v) => v,
        None => return status_host::status(pid, req, E_BAD_LEN, tx),
    };
    let key = SocketKey { pid, handle };
    let Some(kind) = SOCKETS.with(key, |s| s.kind) else {
        return status_host::status(pid, req, E_NO_HANDLE, tx);
    };
    let resolve = |name: &[u8]| dns::resolve_a(state::dns(), name);
    match host_target(kind == Kind::Mixnet, host, direct_chosen, resolve) {
        Ok(ip) => finish_host::finish(pid, req, key, ip, port, tx),
        Err(errno) => {
            if errno == E_NAME_REFUSED {
                mk_debug(NAME_REFUSED.as_ptr(), NAME_REFUSED.len());
            }
            status_host::status(pid, req, errno, tx)
        }
    }
}
