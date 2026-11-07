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

//! The shell behind the loopback host: each event acted on, and each line
//! run by /bin/sh -c with its output sent and recorded.

use super::loopback_host::{urandom, Clock, Host};
use nsh_server::{Event as S, HostSigner, Roles};
use std::process::Command;

pub(super) fn act<K: HostSigner, R: Roles>(
    s: &mut Host<K, R>,
    e: &S,
    line: &mut Vec<u8>,
    clock: &Clock,
) {
    match e {
        S::Admit(a) => s.begin(a.caps, clock.now(), &mut urandom).expect("begin"),
        S::Input(bytes) => {
            line.extend_from_slice(bytes);
            while let Some(at) = line.iter().position(|b| *b == b'\n') {
                let cmd: Vec<u8> = line.drain(..=at).collect();
                run(s, &cmd[..cmd.len() - 1], clock);
            }
        }
        S::Stop(_) => s.exit(0, clock.now(), &mut urandom).expect("exit"),
        _ => {}
    }
}

fn run<K: HostSigner, R: Roles>(s: &mut Host<K, R>, cmd: &[u8], clock: &Clock) {
    s.command(cmd, clock.now(), &mut urandom).expect("command");
    if cmd == b"exit" {
        s.exit(0, clock.now(), &mut urandom).expect("exit");
        return;
    }
    let text = String::from_utf8_lossy(cmd).into_owned();
    let out = Command::new("/bin/sh").arg("-c").arg(&text).output().expect("sh");
    let both = [out.stdout, out.stderr].concat();
    if !both.is_empty() {
        s.output(&both, clock.now(), &mut urandom).expect("output");
    }
}
