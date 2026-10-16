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

//! What Settings may send on through the shell: exactly its five display
//! ops, each to the compositor op of the same meaning with the answer's
//! length the compositor writes; none of the shell's own ops goes through.

use crate::arrange_wire::ARRANGEMENT_LEN;
use crate::ncmp;
use crate::protocol::*;
use crate::relay_table::{relayed, RELAY};

#[test]
fn each_settings_op_becomes_the_compositor_op_of_its_meaning() {
    assert_eq!(relayed(OP_ACCESSIBILITY), Some((ncmp::OP_ACCESSIBILITY, 0)));
    assert_eq!(relayed(OP_ACCESSIBILITY_GET), Some((ncmp::OP_ACCESSIBILITY_GET, 16)));
    assert_eq!(relayed(OP_DISPLAY_LAYOUT), Some((ncmp::OP_OUTPUT_LAYOUT, 0)));
    assert_eq!(relayed(OP_DISPLAY_SCALE), Some((ncmp::OP_OUTPUT_SCALE, 0)));
    let arrangement = (ncmp::OP_OUTPUT_ARRANGEMENT, ARRANGEMENT_LEN);
    assert_eq!(relayed(OP_DISPLAY_ARRANGEMENT), Some(arrangement));
}

#[test]
fn none_of_the_shells_own_ops_is_sent_on() {
    let own = [
        OP_HEALTHCHECK,
        OP_TRAY_REGISTER,
        OP_TRAY_UPDATE,
        OP_TRAY_REMOVE,
        OP_NOTIFY,
        OP_SPOTLIGHT_OPEN,
        OP_OPEN_WITH,
        OP_TAKE_OPEN_ARG,
    ];
    for op in own {
        assert_eq!(relayed(op), None, "op {op:#x}");
    }
    assert_eq!(RELAY.len(), 5);
    assert_eq!(relayed(0x000E), None, "nothing past the last");
}
