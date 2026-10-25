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

//! S-13b: each package op refuses a sender the kernel says lacks the right
//! and serves one that holds it; the question names the sender's pid and its
//! message's time; the other ops ask nothing; and the dispatcher gates first.

use crate::mounts::caps_of::{asked, hold};
use crate::mounts::gate::{admits, needs, Sender};
use crate::protocol::{EPERM, OP_HEALTHCHECK, OP_INSTALL, OP_LIST_INSTALLED, OP_LOAD_BY_NAME};
use crate::protocol::{OP_PKG_COMMIT, OP_PKG_QUERY, OP_PKG_REMOVE};
use crate::{CAPS_OF_APP_INSTALL as INSTALL, CAPS_OF_SPAWN_WINDOW as WINDOW};

const S: Sender = Sender { pid: 40, sent_ms: 1234 };
const DISPATCH: &str = include_str!("../../../capsule_installer/src/server/dispatch.rs");
const RUNNER: &str = include_str!("../../../capsule_installer/src/server/runner.rs");

#[test]
fn each_package_op_needs_a_launcher_and_asks_about_its_own_sender() {
    for op in [OP_PKG_QUERY, OP_PKG_COMMIT, OP_PKG_REMOVE] {
        for (mask, ok) in [(0, false), (INSTALL, false), (WINDOW, true), (u64::MAX, true)] {
            hold(&[(40, mask)]);
            assert_eq!(admits(op, S), ok, "op {op} mask {mask:#x}");
            assert_eq!(asked(), [(40, WINDOW, 1234)], "the sender, at its message's time");
        }
        hold(&[]);
        assert!(!admits(op, S), "no live sender, no service");
    }
}

#[test]
fn a_paid_install_needs_app_install_or_a_launcher() {
    for (mask, ok) in [(0, false), (1 << 9, false), (INSTALL, true), (WINDOW, true)] {
        hold(&[(40, mask)]);
        assert_eq!(admits(OP_INSTALL, S), ok, "mask {mask:#x}");
    }
    assert_eq!(needs(OP_INSTALL), Some(&[INSTALL, WINDOW][..]));
}

#[test]
fn the_other_ops_ask_nothing() {
    for op in [OP_HEALTHCHECK, OP_LIST_INSTALLED, OP_LOAD_BY_NAME, 3, 9, 0xFFFF] {
        hold(&[]);
        assert!(admits(op, S) && asked().is_empty(), "op {op}");
    }
}

#[test]
fn the_dispatcher_gates_before_any_op_and_the_runner_keeps_the_time() {
    assert_eq!(EPERM, -1, "a refused sender is told EPERM");
    let d: String = DISPATCH.split_whitespace().collect();
    let gate = d.find("if!admits(req.op,sender){returnencode_response(req.seq,EPERM,&[]);}");
    assert!(gate.expect("the gate") < d.find("matchreq.op{").expect("the ops"));
    let r: String = RUNNER.split_whitespace().collect();
    assert!(r.contains("mk_ipc_recv_from_at(0,buf.as_mut_ptr(),MAX_MSG,0,&mutpid,&mutsent_ms)"));
    assert!(r.contains("dispatch(req,Sender{pid,sent_ms},&mutkept)"));
}
