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

//! Every value is net.anon's own.

use crate::anon_server_end::REASON_DONE as SERVER_DONE;
use crate::anon_server_errno as server;
use crate::anon_server_ops as server_ops;
use crate::linux::net::anon_ops::*;

const LIMITS: &str = include_str!("../../../../../capsule_net_anon/src/protocol/limits.rs");
pub(super) const RESPOND: &str =
    include_str!("../../../../../capsule_net_anon/src/server/respond.rs");
pub(super) const PARSE: &str =
    include_str!("../../../../../capsule_net_anon/src/server/parse_req.rs");
const TABLE: &str = include_str!("../../../../../capsule_net_anon/src/stream/table.rs");

#[test]
fn every_value_is_net_anons_own() {
    assert_eq!(
        [OP_OPEN_STREAM, OP_SEND, OP_RECV, OP_CLOSE_STREAM],
        [
            server_ops::OP_OPEN_STREAM,
            server_ops::OP_SEND,
            server_ops::OP_RECV,
            server_ops::OP_CLOSE_STREAM
        ]
    );
    assert_eq!(
        [E_OK, E_BAD_LEN, E_NO_TCP, E_NO_DIRECTORY, E_NO_PATH, E_NO_LINK, E_TABLE_FULL],
        [
            server::E_OK,
            server::E_BAD_LEN,
            server::E_NO_TCP,
            server::E_NO_DIRECTORY,
            server::E_NO_PATH,
            server::E_NO_LINK,
            server::E_TABLE_FULL
        ]
    );
    assert_eq!(
        [E_NO_CIRCUIT, E_NO_STREAM, E_RX_EMPTY, E_STREAM_CLOSED, E_DIRECTORY_STALE, E_WOULD_BLOCK],
        [
            server::E_NO_CIRCUIT,
            server::E_NO_STREAM,
            server::E_RX_EMPTY,
            server::E_STREAM_CLOSED,
            server::E_DIRECTORY_STALE,
            server::E_WOULD_BLOCK
        ]
    );
    assert_eq!(REASON_DONE, SERVER_DONE);
    assert!(TABLE.contains(&format!("pub const REASON_DESTROY: u8 = {REASON_DESTROY};")));
    assert!(LIMITS.contains("pub const MAGIC: u32 = 0x414E_4F31;"));
    assert_eq!(MAGIC, 0x414E_4F31);
    assert!(LIMITS.contains(&format!("pub const VERSION: u16 = {VERSION};")));
    assert!(LIMITS.contains(&format!("pub const HDR_LEN: usize = {HDR_LEN};")));
    assert!(LIMITS.contains("pub const IPC_PAYLOAD_MAX: usize = 32 * 1024;"));
    assert_eq!(PAYLOAD_MAX, 32 * 1024);
}
