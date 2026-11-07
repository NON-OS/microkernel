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

//! `ping HOST`: PING_COUNT ICMP echoes, each said, then the totals, as the
//! terminal's `ping` prints them. ICMP crosses no anonymity network, so the
//! host sends one only on the Direct route and says why not otherwise. A
//! name is looked up first, under the same rule; an address is used as it
//! is.

use alloc::vec::Vec;

use super::dns::{failed, is_host, parse_ipv4, push_ip, HOST_RULE};
use crate::host::{Echo, Host, Resolve};
use crate::out::ShellOut;
use crate::util::format_u64;

/// Echoes one `ping` sends. Each is waited for at most the host's bound,
/// so a ping holds the session a few seconds at most.
pub const PING_COUNT: u16 = 4;
/// What an echo off Direct would have done.
pub const LEAVES_DIRECTLY: &[u8] = b"ICMP cannot cross it and would leave directly";

pub fn run(out: &mut dyn ShellOut, host: &mut dyn Host, w: &[&[u8]]) -> bool {
    let [_, name] = w else {
        out.writeln(b"usage: ping <host>");
        return false;
    };
    let dst = match parse_ipv4(name) {
        Some(ip) => ip,
        None if !is_host(name) => {
            out.writeln(&[b"ping".as_slice(), HOST_RULE].concat());
            return false;
        }
        None => match host.resolve(name) {
            Resolve::Addr(ip) => ip,
            other => {
                out.writeln(&failed(b"ping", name, other));
                return false;
            }
        },
    };
    let mut head = b"PING ".to_vec();
    head.extend_from_slice(name);
    head.extend_from_slice(b" (");
    push_ip(&mut head, &dst);
    head.push(b')');
    let mut stats = Stats::default();
    for seq in 1..=PING_COUNT {
        let echo = host.echo(dst, seq);
        if seq == 1 && !matches!(echo, Echo::Refused(_) | Echo::Unavailable) {
            out.writeln(&head);
        }
        match echo {
            Echo::Reply(ms) => {
                stats.got(ms);
                let mut line = b"reply from ".to_vec();
                push_ip(&mut line, &dst);
                line.extend_from_slice(b": seq=");
                push(&mut line, u64::from(seq));
                line.extend_from_slice(b" time=");
                push(&mut line, ms);
                line.extend_from_slice(b" ms");
                out.writeln(&line);
            }
            Echo::Timeout | Echo::Unreachable => {
                stats.sent += 1;
                let mut line = b"request timed out: seq=".to_vec();
                push(&mut line, u64::from(seq));
                out.writeln(&line);
            }
            fatal => {
                out.writeln(&stopped(fatal));
                if seq == 1 {
                    return false;
                }
                break;
            }
        }
    }
    summary(out, name, &stats);
    stats.recv > 0
}

/// The line an echo that stops the ping prints; a fault that will not fix
/// itself between packets is said once.
fn stopped(e: Echo) -> Vec<u8> {
    match e {
        Echo::Refused(why) => {
            let mut line = b"ping: ".to_vec();
            line.extend_from_slice(why.as_bytes());
            line.extend_from_slice(b"; ");
            line.extend_from_slice(LEAVES_DIRECTLY);
            line.extend_from_slice(b", so nothing was sent");
            line
        }
        Echo::NoRoute => b"no route to host".to_vec(),
        Echo::NotReady => b"ping: network not ready".to_vec(),
        Echo::Unavailable => b"ping: net unavailable".to_vec(),
        _ => b"ping: send failed".to_vec(),
    }
}

#[derive(Default)]
struct Stats {
    sent: u64,
    recv: u64,
    min: Option<u64>,
    max: u64,
    sum: u64,
}

impl Stats {
    fn got(&mut self, ms: u64) {
        self.sent += 1;
        self.recv += 1;
        self.sum = self.sum.saturating_add(ms);
        self.min = Some(self.min.map_or(ms, |m| m.min(ms)));
        self.max = self.max.max(ms);
    }
}

fn summary(out: &mut dyn ShellOut, name: &[u8], s: &Stats) {
    let mut head = b"--- ".to_vec();
    head.extend_from_slice(name);
    head.extend_from_slice(b" ping statistics ---");
    out.writeln(&head);
    let mut counts = Vec::new();
    push(&mut counts, s.sent);
    counts.extend_from_slice(b" transmitted, ");
    push(&mut counts, s.recv);
    counts.extend_from_slice(b" received, ");
    let lost = s.sent.saturating_sub(s.recv).saturating_mul(100).checked_div(s.sent);
    push(&mut counts, lost.unwrap_or(0));
    counts.extend_from_slice(b"% loss");
    out.writeln(&counts);
    if let (Some(min), Some(avg)) = (s.min, s.sum.checked_div(s.recv)) {
        let mut rtt = b"rtt min/avg/max = ".to_vec();
        push(&mut rtt, min);
        rtt.push(b'/');
        push(&mut rtt, avg);
        rtt.push(b'/');
        push(&mut rtt, s.max);
        rtt.extend_from_slice(b" ms");
        out.writeln(&rtt);
    }
}

fn push(line: &mut Vec<u8>, v: u64) {
    let mut num = [0u8; 20];
    let n = format_u64(v, &mut num);
    line.extend_from_slice(&num[..n]);
}
