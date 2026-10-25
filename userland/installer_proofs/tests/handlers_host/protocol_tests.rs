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

//! The installer's wire numbers, as its peers speak them.
//
// The real protocol module is mounted whole (the frames here are decoded by
// its decode_request), so its whole table is pinned: renumbering an op or the
// reply endpoint breaks the market driver and the kernel, which hold copies.

use crate::protocol::types::OP_LOAD_FROM_STORE;
use crate::protocol::{
    EINVAL, KERNEL_REPLY_ENDPOINT, OP_HEALTHCHECK, OP_INSTALL, OP_LIST_INSTALLED, OP_LOAD_BY_NAME,
    OP_PKG_COMMIT, OP_PKG_QUERY, OP_PKG_REMOVE,
};

#[test]
fn the_op_table_is_one_to_eight_with_the_loads_at_three_and_four() {
    let ops = [
        OP_HEALTHCHECK,
        OP_INSTALL,
        OP_LOAD_FROM_STORE,
        OP_LOAD_BY_NAME,
        OP_LIST_INSTALLED,
        OP_PKG_QUERY,
        OP_PKG_COMMIT,
        OP_PKG_REMOVE,
    ];
    assert_eq!(ops, [1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn the_refusal_status_and_reply_endpoint_are_pinned() {
    assert_eq!(EINVAL, -22);
    assert_eq!(KERNEL_REPLY_ENDPOINT, 0x1_0000_001A);
}
