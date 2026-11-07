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

//! The commands a session answers to, each a shared builtin (`nslookup` and
//! `ping` ask the host, which answers them only on the Direct route), and
//! the few that belong to the session itself: `true`, `false`, `exit`,
//! `help`, `set`, `unset` and `env` over the session's variables, `alias`,
//! `unalias` and `type` (also `which`), `whoami`, `history`, `clear`, and
//! `NAME=VALUE`.

use super::history::history;
use super::names::{alias, kind, unalias};
use super::release::version;
use super::script::is_name;
use super::who::{role_allows, whoami};
use super::{Session, VAR_REFUSED};
use crate::builtin::{
    attest, b64, clock, diff, dns, echo, ed25519, hash, hex, mem, path, ping, printf, ps, rand,
    seq, service, test,
};
use crate::host::Host;
use crate::out::ShellOut;

pub(super) enum Done {
    Status(i32),
    Exit(i32),
}

/// Every name this shell runs, as `help` lists them.
pub const NAMES: &[&[u8]] = &[
    b"alias",
    b"attest",
    b"b64",
    b"basename",
    b"clear",
    b"commands",
    b"date",
    b"diff",
    b"dirname",
    b"echo",
    b"ed25519",
    b"env",
    b"exit",
    b"false",
    b"hash",
    b"help",
    b"hex",
    b"history",
    b"mem",
    b"nslookup",
    b"pcr",
    b"ping",
    b"printf",
    b"ps",
    b"rand",
    b"seq",
    b"service",
    b"set",
    b"svc",
    b"test",
    b"true",
    b"type",
    b"unalias",
    b"unset",
    b"uptime",
    b"version",
    b"which",
    b"whoami",
];

pub(super) fn call(
    s: &mut Session,
    out: &mut dyn ShellOut,
    host: &mut dyn Host,
    w: &[&[u8]],
) -> Done {
    let Some(name) = w.first() else { return Done::Status(0) };
    if !role_allows(s.who.as_ref(), name, out) {
        return Done::Status(1);
    }
    let ok = match *name {
        b"echo" => {
            echo::run(out, w);
            true
        }
        b"printf" => printf::run(out, w),
        b"seq" => seq::run(out, w),
        b"hex" => hex::run(out, w),
        b"b64" => b64::run(out, w),
        b"hash" => hash::run(out, w),
        b"rand" => rand::run(out, host, w),
        b"basename" => path::basename(out, w),
        b"dirname" => path::dirname(out, w),
        b"diff" => diff::run(out, host, w),
        b"date" => clock::date(out, host),
        b"uptime" => clock::uptime(out, host),
        b"mem" => mem::run(out, host),
        b"attest" => attest::run(out, host),
        b"pcr" => attest::pcr(out, host),
        b"ps" => ps::run(out, host, w),
        b"service" | b"svc" => service::run(out, host, w),
        b"nslookup" => dns::nslookup(out, host, w),
        b"ping" => ping::run(out, host, w),
        b"ed25519" => ed25519::run(out, host, w),
        b"true" => true,
        b"false" => false,
        b"test" => return Done::Status(test::run(out, w)),
        b"exit" => return exit(out, w),
        b"help" | b"commands" => help(out),
        b"version" => version(out, w),
        b"env" => env(s, out),
        b"set" => set(s, out, w),
        b"unset" => unset(s, out, w),
        b"alias" => alias(s, out, w),
        b"unalias" => unalias(s, out, w),
        b"type" | b"which" => kind(s, out, w),
        b"whoami" => whoami(s.who.as_ref(), out),
        b"history" => history(s, out, w),
        b"clear" => clear(out),
        other => return assign(s, out, other, w.len()),
    };
    Done::Status(i32::from(!ok))
}

fn exit(out: &mut dyn ShellOut, w: &[&[u8]]) -> Done {
    match w.get(1).map(|c| core::str::from_utf8(c).ok().and_then(|c| c.parse::<i32>().ok())) {
        None => Done::Exit(0),
        Some(Some(code)) if w.len() == 2 => Done::Exit(code),
        _ => {
            out.writeln(b"usage: exit [status]");
            Done::Status(2)
        }
    }
}

fn help(out: &mut dyn ShellOut) -> bool {
    out.writeln(&NAMES.join(&b' '));
    out.writeln(b"control: if ...; then ...; else ...; fi   for N in ...; do ...; done   while ...; do ...; done");
    out.writeln(b"filters after |: grep sort uniq cut nl wc head tail tac rev");
    true
}

/// The cursor home and the screen erased, for the terminal at the other
/// end; a session keeps no screen of its own.
pub const CLEAR: &[u8] = b"\x1b[H\x1b[2J";

fn clear(out: &mut dyn ShellOut) -> bool {
    out.writeln(CLEAR);
    true
}

fn env(s: &Session, out: &mut dyn ShellOut) -> bool {
    for (n, v) in &s.vars {
        out.writeln(&[&n[..], b"=", &v[..]].concat());
    }
    true
}

fn set(s: &mut Session, out: &mut dyn ShellOut, w: &[&[u8]]) -> bool {
    match w {
        [_, name, value @ ..] if is_name(name) => {
            let set = s.set_var(name, &value.join(&b' '));
            if !set {
                out.writeln(VAR_REFUSED);
            }
            set
        }
        _ => {
            out.writeln(b"usage: set NAME [VALUE...]");
            false
        }
    }
}

fn unset(s: &mut Session, out: &mut dyn ShellOut, w: &[&[u8]]) -> bool {
    match w {
        [_, name] => {
            s.vars.retain(|(n, _)| n != name);
            true
        }
        _ => {
            out.writeln(b"usage: unset NAME");
            false
        }
    }
}

/// `NAME=VALUE` alone sets a variable; any other word is not a command.
fn assign(s: &mut Session, out: &mut dyn ShellOut, word: &[u8], words: usize) -> Done {
    if let Some(eq) = word.iter().position(|c| *c == b'=') {
        if words == 1 && is_name(&word[..eq]) {
            if s.set_var(&word[..eq], &word[eq + 1..]) {
                return Done::Status(0);
            }
            out.writeln(VAR_REFUSED);
            return Done::Status(1);
        }
    }
    let mut l = word.to_vec();
    l.extend_from_slice(b": command not found");
    out.writeln(&l);
    Done::Status(127)
}
