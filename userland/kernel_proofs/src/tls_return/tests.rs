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


const PEER_TLS: &str = include_str!("../../../../src/process/foreign/peer_tls.rs");
const TRAP_WAIT: &str = include_str!("../../../../src/process/foreign/trap_wait.rs");
const SWITCH: &str = include_str!("../../../../src/arch/x86_64/context/switch/dispatch.rs");
const SIGNAL_ENTER: &str = include_str!("../../../../src/process/foreign/signal_enter.rs");
const EXEC_ENTER: &str = include_str!("../../../../src/process/foreign/exec_enter.rs");
const TICK: &str = include_str!("../../../../src/process/foreign/interrupt/tick.rs");

/// One guest thread's view: what the CPU's FS register holds and what its
/// control block holds.
struct Thread {
    cpu_fs: u64,
    block: u64,
}

impl Thread {
    /// MkPeerTls: the control block only.
    fn peer_tls(&mut self, base: u64) {
        self.block = base;
    }

    /// The scheduler switching the thread back in.
    fn switch_in(&mut self) {
        self.cpu_fs = self.block;
    }

    /// trap_wait's value return, before the fix and after it.
    fn value_return(&mut self, reloads: bool) {
        if reloads {
            self.cpu_fs = self.block;
        }
    }
}

/// An arch_prctl(ARCH_SET_FS) served while the thread is parked, answered
/// with or without a switch, returned with or without the reload.
fn after_set_fs(switched: bool, reloads: bool) -> u64 {
    let mut t = Thread { cpu_fs: 0, block: 0 };
    t.peer_tls(0x7fff_f000);
    if switched {
        t.switch_in();
    }
    t.value_return(reloads);
    t.cpu_fs
}

#[test]
fn a_set_base_reaches_the_cpu_with_or_without_a_switch() {
    for switched in [false, true] {
        assert_eq!(after_set_fs(switched, true), 0x7fff_f000, "switched: {switched}");
    }
}

#[test]
fn without_the_reload_an_unswitched_return_keeps_the_old_base() {
    // The failure the boot showed: FS 0, and a TLS read at -4.
    assert_eq!(after_set_fs(false, false), 0);
    assert_eq!(0u64.wrapping_sub(4), 0xffff_ffff_ffff_fffc);
    assert_eq!(after_set_fs(true, false), 0x7fff_f000);
}

/// The text of the `Answer::Value` arm of trap_wait's settle.
fn value_arm() -> &'static str {
    let start = TRAP_WAIT.find("Answer::Value(value) =>").expect("settle answers a value");
    let rest = &TRAP_WAIT[start..];
    let end = rest.find("Answer::Execed").expect("the next arm");
    &rest[..end]
}

#[test]
fn peer_tls_writes_only_the_control_block() {
    assert!(PEER_TLS.contains("pcb.set_tls_base(base)"));
    assert!(!PEER_TLS.contains("set_user_tls"));
    assert!(!PEER_TLS.contains("set_fs_base"));
}

#[test]
fn a_switch_installs_the_control_blocks_base() {
    assert!(SWITCH.contains("set_fs_base(pcb.get_tls_base())"));
}

#[test]
fn a_value_return_installs_the_control_blocks_base() {
    let arm = value_arm();
    let read = arm.find("get_tls_base()").expect("reads the control block");
    let install = arm.find("set_user_tls(base)").expect("installs it");
    let back = arm.rfind("value").expect("returns the value");
    assert!(read < install && install < back);
}

#[test]
fn every_other_way_out_installs_its_contexts_base() {
    assert!(SIGNAL_ENTER.contains("set_user_tls(ctx.fs_base)"));
    assert!(EXEC_ENTER.contains("set_user_tls(ctx.fs_base)"));
    // A tick stop entering a handler takes the base the control block holds.
    assert!(TICK.contains("p.get_tls_base()"));
    assert!(TICK.contains("set_user_tls(to.fs_base)"));
}
