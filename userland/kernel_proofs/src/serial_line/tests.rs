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

//! Four CPUs' worth of writers at once: every write the port takes is one
//! whole line, never two lines' pieces; and no diag or boot_log line, nor
//! [USER-FIRST], is written in pieces any more.

use super::line::Line;
use super::print::WRITES;

const DIAG: [&str; 3] = [
    include_str!("../../../../src/sys/diag/mod.rs"),
    include_str!("../../../../src/sys/diag/late.rs"),
    include_str!("../../../../src/sys/diag/reply.rs"),
];
const BOOT_LOG: &str = include_str!("../../../../src/sys/boot_log/output.rs");
const USER: &str = include_str!("../../../../src/process/userspace/asm.rs");

#[test]
fn concurrent_lines_reach_the_port_whole() {
    let writers: Vec<_> = (0..4u64)
        .map(|cpu| {
            std::thread::spawn(move || {
                for i in 0..2000u64 {
                    let mut l = Line::new();
                    l.str(b"[CPU").dec(cpu).str(b"] seq=").dec(i).str(b" rip=").hex(i).end();
                }
            })
        })
        .collect();
    writers.into_iter().for_each(|w| w.join().unwrap());
    let writes = WRITES.lock().unwrap();
    assert_eq!(writes.len(), 8000, "one write a line");
    for w in writes.iter() {
        let s = std::str::from_utf8(w).unwrap();
        let (cpu, rest) = s.strip_prefix("[CPU").and_then(|r| r.split_once("] seq=")).unwrap();
        let (seq, rip) = rest.split_once(" rip=0x").expect(s);
        assert!(cpu.len() == 1 && seq.parse::<u64>().is_ok() && rip.len() == 16, "{s}");
        assert_eq!(u64::from_str_radix(rip, 16).unwrap(), seq.parse::<u64>().unwrap(), "{s}");
    }
}

#[test]
fn no_diag_or_boot_log_line_is_written_in_pieces() {
    for src in DIAG.iter().chain([&BOOT_LOG]) {
        assert!(!src.contains("serial::print"), "a line printed in pieces");
    }
    assert!(DIAG[0].contains("fn trail(l: &mut Line, n: u32) {"), "every diag line ends in trail");
    let first = &USER[USER.find("[USER-FIRST]").unwrap() - 200..];
    assert!(!first[..400].contains("serial::print"), "[USER-FIRST] in one write");
}
