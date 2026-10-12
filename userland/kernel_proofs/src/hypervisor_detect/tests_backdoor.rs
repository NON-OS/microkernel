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

//! The VMware backdoor answers the absolute pointer alone, and only on a
//! host that has one: no RPC, no host time, no guest info, and never a port
//! read on a machine with no hypervisor.

use super::backdoor_rule::*;
use super::vendor::Hypervisor;

#[test]
fn only_hosts_with_a_backdoor_are_asked() {
    for h in [Hypervisor::Vmware, Hypervisor::Kvm, Hypervisor::QemuTcg] {
        assert!(host_answers(h), "{}", h.slug());
    }
    for code in 0..=255u8 {
        let h = Hypervisor::from_code(code);
        if !matches!(h, Hypervisor::Vmware | Hypervisor::Kvm | Hypervisor::QemuTcg) {
            assert!(!host_answers(h), "{}", h.slug());
        }
    }
}

#[test]
fn the_pointer_calls_are_allowed() {
    assert!(call_allowed(CMD_GETVERSION, 0));
    assert!(call_allowed(CMD_ABSPOINTER_STATUS, 0));
    for n in 1..=4 {
        assert!(call_allowed(CMD_ABSPOINTER_DATA, n));
    }
    for sub in [ABSPOINTER_ENABLE, ABSPOINTER_DISABLE, ABSPOINTER_RELATIVE, ABSPOINTER_ABSOLUTE] {
        assert!(call_allowed(CMD_ABSPOINTER_COMMAND, sub));
    }
}

#[test]
fn a_data_read_is_one_to_four_words() {
    assert!(!call_allowed(CMD_ABSPOINTER_DATA, 0));
    assert!(!call_allowed(CMD_ABSPOINTER_DATA, 5));
    assert!(!call_allowed(CMD_ABSPOINTER_DATA, u32::MAX));
}

#[test]
fn other_pointer_commands_are_refused() {
    for sub in [0, 1, 0x4C45_5253, 0x5342_4153, u32::MAX] {
        assert!(!call_allowed(CMD_ABSPOINTER_COMMAND, sub), "{sub:#x}");
    }
}

#[test]
fn every_other_command_is_refused() {
    // 0x1E is RPC, 0x17 the host's time, 0x0B the guest info, 0x06 the
    // clipboard length: each moves data between guest and host.
    for cmd in (0..=0xFFu32).chain([0x1_000A, 0x1_0027, u32::MAX]) {
        if matches!(cmd, 10 | 39 | 40 | 41) {
            continue;
        }
        for arg in [0, 1, 4, ABSPOINTER_ENABLE] {
            assert!(!call_allowed(cmd, arg), "cmd {cmd:#x}");
        }
    }
}

#[test]
fn the_port_and_magic_are_vmware_s() {
    assert_eq!(BACKDOOR_PORT, 0x5658, "'VX'");
    assert_eq!(BACKDOOR_MAGIC.to_be_bytes(), *b"VMXh");
}

#[test]
fn only_the_magic_back_is_an_answer() {
    assert!(answered([6, BACKDOOR_MAGIC, CMD_GETVERSION, 0x5658]));
    assert!(!answered([u32::MAX; 4]), "an unassigned port");
    assert!(!answered([u32::MAX, BACKDOOR_MAGIC, 0, 0]));
    assert!(!answered([6, 0, 0, 0]));
}
