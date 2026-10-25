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

//! The handler feeds the rule the kernel's own facts: the caller's token for
//! SpawnBroker, the pid's state, its start time on the message's clock, its
//! mask; and the cap table admits only a SpawnBroker holder to the call.

const HANDLER: &str = include_str!("../../../../../../src/syscall/microkernel/caps_of/mod.rs");
const TABLE: &str = include_str!("../../../../../../src/syscall/contract/cap_table/mk_proc.rs");

fn flat(s: &str) -> String {
    s.lines().map(|l| l.split("//").next().unwrap()).collect::<String>().split_whitespace().collect()
}

#[test]
fn the_handler_asks_the_rule_with_the_kernel_s_facts() {
    let h = flat(HANDLER);
    for part in [
        "letbroker=caps.is_valid()&&caps.grants(Capability::SpawnBroker);",
        "ProcessState::Zombie(_)|ProcessState::Terminated(_)",
        "started_ms:enqueued_uptime(now.0,now.1,pcb.start_time_ms.load(Ordering::Relaxed)),",
        "caps:pcb.caps_bits.load(Ordering::Acquire),",
        "answer(broker,bit,target,sent_ms)",
    ] {
        assert!(h.contains(part), "the handler lost `{part}`");
    }
}

#[test]
fn only_a_spawn_broker_holder_reaches_the_call() {
    let t = flat(TABLE);
    assert!(t.contains("SyscallNumber::MkCapsOf=>caps.is_valid()&&caps.grants(Capability::SpawnBroker),"));
}
