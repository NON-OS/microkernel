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

//! `service NAME...` (also `svc`): where each named service is served, as
//! the kernel's registry says, one line a name: its port and the pid that
//! serves it. A name is held to what the registry takes, 1 to 64 bytes of
//! printable ASCII with no space, and checked before the host is asked, so
//! no byte of it reaches the kernel or the terminal at the other end of a
//! session as anything else. Fails if any name is not registered.

use alloc::vec::Vec;

use crate::host::{Host, Service};
use crate::out::ShellOut;
use crate::util::format_u64;

/// The registry's longest name.
pub const NAME_MAX: usize = 64;
pub const USAGE: &[u8] = b"usage: service NAME...";
pub const BAD_NAME: &[u8] =
    b"service: a service name is 1 to 64 bytes of printable ASCII with no space";
pub const UNANSWERED: &[u8] = b"service: this host cannot read the service registry";
pub const ABSENT: &[u8] = b": not registered";

pub fn run(out: &mut dyn ShellOut, host: &mut dyn Host, w: &[&[u8]]) -> bool {
    let names = w.get(1..).unwrap_or_default();
    if names.is_empty() {
        out.writeln(USAGE);
        return false;
    }
    if !names.iter().all(|n| valid(n)) {
        out.writeln(BAD_NAME);
        return false;
    }
    let mut all = true;
    for name in names {
        match host.service(name) {
            Service::Found { port, pid } => out.writeln(&found(name, port, pid)),
            Service::Absent => {
                all = false;
                out.writeln(&[name, ABSENT].concat());
            }
            Service::Unanswered => {
                out.writeln(UNANSWERED);
                return false;
            }
        }
    }
    all
}

/// A name the registry could hold and the terminal shows as itself.
pub fn valid(name: &[u8]) -> bool {
    (1..=NAME_MAX).contains(&name.len()) && name.iter().all(|c| (0x21..0x7f).contains(c))
}

fn found(name: &[u8], port: u32, pid: u32) -> Vec<u8> {
    let mut l = Vec::with_capacity(name.len() + 32);
    l.extend_from_slice(name);
    l.extend_from_slice(b"  port=");
    num(&mut l, port);
    l.extend_from_slice(b"  pid=");
    num(&mut l, pid);
    l
}

fn num(l: &mut Vec<u8>, v: u32) {
    let mut buf = [0u8; 20];
    let n = format_u64(u64::from(v), &mut buf);
    l.extend_from_slice(&buf[..n]);
}
