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
//! One line on how a join went.
use crate::firmware::gen3::join::exchange::Progress;
use crate::firmware::gen3::join::run::{JoinEnd, Joined};

use super::super::say::Line;

// One line on how a join went.
pub(super) fn say_join(p: &Progress, out: &Result<Joined, JoinEnd>) {
    let mut l = Line::new();
    match out {
        Ok(_) => l.text(b"join: port open"),
        Err(JoinEnd::Refused(f)) => {
            let (code, ap) = crate::server::join_wire::failure_code(*f);
            l.text(b"join: refused, code ").num(code.unsigned_abs()).text(b" status ").num(u32::from(ap))
        }
        Err(JoinEnd::TimedOut) => l.text(b"join: the access point stopped answering"),
        Err(JoinEnd::NotTheNetwork) => l.text(b"join: the beacon names no usable channel"),
        Err(JoinEnd::Setup(e)) => l.text(b"join: a firmware context did not go up: ").text(setup_text(e)),
        Err(JoinEnd::Firmware(Some(c))) => l.text(b"join: command ").hex(u32::from(c.group) << 8 | u32::from(c.cmd)).text(b" failed"),
        Err(JoinEnd::Firmware(None)) => l.text(b"join: the firmware raised its error cause"),
        Err(JoinEnd::Keys(_)) => l.text(b"join: a key did not go into the firmware"),
    };
    l.text(b"; sent ").num(p.sent).text(b" recv ").num(p.recv).text(b" eapol ").num(p.eapol);
    l.text(b" state ").num(u32::from(p.state)).text(b" refused ").num(p.refused).send();
}
pub(super) fn setup_text(e: &crate::firmware::gen3::join::bss::SetupError) -> &'static [u8] {
    use crate::firmware::gen3::join::bss::SetupError;
    use crate::firmware::gen3::station::session::Session;
    match e {
        SetupError::Command(c) if c.why == crate::firmware::gen3::dev::WaitError::FirmwareError => b"firmware error",
        SetupError::Command(_) => b"a command went unanswered",
        SetupError::BadQueueReply => b"a malformed queue reply",
        SetupError::BadQueue(_) => b"a queue number the device cannot have",
        SetupError::NoSession(Some(Session::Refused)) => b"session protection refused",
        SetupError::NoSession(_) => b"never on channel",
        SetupError::Region => b"the transmit region",
    }
}
