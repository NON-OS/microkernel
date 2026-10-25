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

//! OP_LOAD_FROM_STORE is retired: op 3 is refused with EINVAL and loads
//! nothing, its number stays taken, and the dispatcher asks `retired` before
//! it reads any op, with no arm left for 3 and no handler behind it.

use crate::frame::{einval, frame};
use crate::protocol::retired::RETIRED;
use crate::protocol::types::OP_LOAD_FROM_STORE;
use crate::protocol::{decode_request, retired};
use crate::syscall;

const DISPATCH: &str = include_str!("../../../capsule_installer/src/server/dispatch.rs");
const HANDLERS: &str = include_str!("../../../capsule_installer/src/server/handlers/mod.rs");

#[test]
fn op_3_is_refused_and_loads_nothing() {
    assert_eq!((OP_LOAD_FROM_STORE, RETIRED), (3, [3]));
    syscall::reset(5);
    for payload in [&[][..], &[0u8; 28], &[0xFF; 200]] {
        let f = frame(3, payload);
        let req = decode_request(&f).expect("frame decodes");
        assert_eq!(retired(&req), Some(einval()));
    }
    assert!(syscall::take().is_empty(), "no load was made");
    for op in [1u16, 2, 4, 5, 6, 7, 8] {
        let f = frame(op, &[]);
        assert_eq!(retired(&decode_request(&f).unwrap()), None, "op {op} is not retired");
    }
}

#[test]
fn the_dispatcher_refuses_retired_ops_first_and_keeps_no_handler_for_3() {
    let code: String = DISPATCH.split_whitespace().collect();
    let first = code.find("ifletSome(refused)=retired(&req){returnrefused;}").expect("asks retired");
    assert!(first < code.find("matchreq.op{").expect("then the ops"));
    assert!(!code.contains("OP_LOAD_FROM_STORE"), "no arm for op 3");
    assert!(!HANDLERS.contains("load_store"), "no handler behind it");
}
