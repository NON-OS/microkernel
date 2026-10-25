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

//! K20 on the GOP path: the compositor takes its own surface's handle by
//! sharing it with the process its own name resolves to, which the kernel
//! answers with the owner's handle and no receiver, so it never asks for its
//! pid (MkGetPid, CoreExec) and grants no write right to anyone.

#[path = "../../../compositor/src/setup/prime_gop_handle.rs"]
mod prime_gop_handle;

use std::cell::RefCell;

use prime_gop_handle::own_handle;

const OWN: u32 = 7;

#[test]
fn the_share_names_the_process_holding_the_compositor_name_read_only() {
    let (asked, shared) = (RefCell::new(Vec::new()), RefCell::new(Vec::new()));
    let lookup = |name: &[u8]| {
        asked.borrow_mut().push(name.to_vec());
        Some(OWN)
    };
    let share = |sid, pid, flags| {
        shared.borrow_mut().push((sid, pid, flags));
        0x4000 + sid as i64
    };
    assert_eq!(own_handle(0x21, lookup, share), Ok(0x4021), "not the handle the share answers");
    assert_eq!(asked.into_inner(), [b"compositor".to_vec()], "asked under another name");
    assert_eq!(shared.into_inner(), [(0x21, OWN, 0)], "a right granted, or another pid named");
}

#[test]
fn no_registered_name_shares_nothing() {
    let shared = RefCell::new(0);
    let share = |_, _, _| {
        *shared.borrow_mut() += 1;
        1
    };
    assert!(own_handle(3, |_| None, share).is_err());
    assert_eq!(shared.into_inner(), 0, "a share with no receiver named");
}

#[test]
fn a_refused_share_is_no_handle() {
    for refused in [0, -22, -3] {
        assert!(own_handle(3, |_| Some(OWN), |_, _, _| refused).is_err(), "{refused}");
    }
}
