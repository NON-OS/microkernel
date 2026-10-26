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

//! A fatal line is written even when the port's lock is held and never freed
//! (a CPU that faulted holding it), within its bound; and when the lock is
//! free it is held for the whole line, so no other writer lands inside it.

use std::sync::Mutex;
use std::time::Duration;

use super::fatal::write_bounded;

#[test]
fn a_held_lock_never_stops_the_fatal_line() {
    static PORT: Mutex<()> = Mutex::new(());
    let _held_forever = PORT.lock().unwrap();
    let (done, line) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut out = Vec::new();
        write_bounded(1 << 16, || PORT.try_lock().ok(), |b| out.push(b), &[b"[TRAP PF] cpl=0"]);
        let _ = done.send(out);
    });
    let out = line.recv_timeout(Duration::from_secs(10)).expect("the fatal line never hangs");
    assert_eq!(out, b"[TRAP PF] cpl=0\r\n", "written whole, lock or no lock");
}

#[test]
fn a_free_lock_is_held_for_the_whole_fatal_line() {
    let port = Mutex::new(());
    let mut held = Vec::new();
    let parts: [&[u8]; 2] = [b"\r\n", b"[PANIC] x"];
    write_bounded(8, || port.try_lock().ok(), |_| held.push(port.try_lock().is_err()), &parts);
    assert_eq!(held.len(), 2 + 9 + 2, "the opening, the line and its newline");
    assert!(held.iter().all(|&h| h), "every byte under the lock");
    assert!(port.try_lock().is_ok(), "and the lock is given back");
}

/// The trap, panic and stop lines: each one whole, through the fatal writer.
const FATAL: [&str; 7] = [
    include_str!("../../../../src/arch/x86_64/diag/dump_trap.rs"),
    include_str!("../../../../src/arch/x86_64/diag/walk_fault.rs"),
    include_str!("../../../../src/arch/x86_64/diag/fatal_notice.rs"),
    include_str!("../../../../src/interrupts/handlers/exceptions/gpf.rs"),
    include_str!("../../../../src/boot/stop.rs"),
    include_str!("../../../../src/entry/oom.rs"),
    include_str!("../../../../src/smp/cpu_unregistered.rs"),
];

#[test]
fn every_fatal_line_goes_whole_through_the_fatal_writer() {
    for (i, src) in FATAL.iter().enumerate() {
        assert!(src.contains(".end_fatal();"), "fatal file {i} writes through end_fatal");
        assert!(!src.contains("serial::print("), "fatal file {i} prints no piece");
        assert!(!src.contains("print_hex_u64("), "fatal file {i} prints no hex piece");
    }
}
