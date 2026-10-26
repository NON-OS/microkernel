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

//! Who may power the machine off, and the shell's frame against the
//! capsule's: the request the shell sends is the one the capsule accepts, and
//! the refusal the capsule writes is one the shell reads as a refusal.

use crate::frame;
use crate::rule::{op_allowed, verdict, Caller, Verdict, EINVAL, EPERM, INSTALLERS};
use crate::wire;

#[test]
fn only_a_well_formed_request_from_the_shell_powers_off() {
    assert_eq!(verdict(true, true), Verdict::Off);
    assert_eq!(verdict(true, false), Verdict::Refuse(EPERM));
    assert_eq!(verdict(false, true), Verdict::Refuse(EINVAL));
    assert_eq!(verdict(false, false), Verdict::Refuse(EINVAL));
}

#[test]
fn reboot_is_the_shells_and_the_installers_and_shutdown_the_shells() {
    for op in 0..=u16::MAX {
        assert!(op_allowed(op, Caller::Shell), "op {op} from the shell");
        assert_eq!(op_allowed(op, Caller::Installer), op != 3, "op {op} from an installer");
        assert_eq!(op_allowed(op, Caller::Other), op != 2 && op != 3, "op {op} from anyone else");
    }
    let only_two: [&[u8]; 2] = [b"app.install", b"tool.install"];
    assert_eq!(INSTALLERS, only_two, "S-14: exactly the two installers");
}

#[test]
fn the_shells_request_is_the_frame_the_capsule_accepts() {
    for id in [0, 1, 0x1234_5678, u32::MAX] {
        let req = frame::request(id);
        assert!(wire::is_poff(&req) && wire::well_formed(&req), "request id {id:#x}");
    }
    assert_eq!(frame::MAGIC, wire::MAGIC);
    assert_eq!(
        (frame::VERSION, frame::OP_OFF, frame::HDR_LEN),
        (wire::VERSION, wire::OP_OFF, wire::HEADER)
    );
}

#[test]
fn a_refusal_reads_back_as_a_refusal_to_the_same_request() {
    for status in [EPERM, EINVAL, -38] {
        let req = frame::request(77);
        let reply = wire::refusal(&req, status);
        let n = reply.len() as i64;
        assert_eq!(frame::heard(n, &reply, 77), frame::Heard::Refused(status));
        assert_eq!(frame::heard(n, &reply, 78), frame::Heard::Garbled, "another request's id");
    }
}

#[test]
fn the_power_protocol_is_never_taken_for_a_power_off() {
    // capsule_power's own frames start with its magic, "RWOP".
    let power = 0x504F_5752u32.to_le_bytes();
    assert!(!wire::is_poff(&power));
}
