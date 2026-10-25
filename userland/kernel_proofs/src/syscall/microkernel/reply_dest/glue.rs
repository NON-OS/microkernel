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

//! The send path asks the rule, with the kernel's own facts, before it hands
//! an answer to a pending caller, and a refusal is said without a pid.

const SEND: &str = include_str!("../../../../../../src/syscall/microkernel/ipc/send_reply.rs");
const FACTS: &str =
    include_str!("../../../../../../src/syscall/microkernel/ipc/reply_dest_facts.rs");

fn flat(s: &str) -> String {
    s.lines()
        .map(|l| l.split("//").next().unwrap())
        .collect::<String>()
        .split_whitespace()
        .collect()
}

#[test]
fn to_caller_asks_the_rule_and_no_endpoint_gate() {
    let s = flat(SEND);
    assert!(s.contains(
        "if!super::reply_dest_facts::admits(pid,caller_inbox,caller_pid){returnERRNO_PERM;}"
    ));
    assert!(!s.contains("caller_satisfies_endpoint"));
}

#[test]
fn the_facts_are_the_kernel_s_own() {
    let f = flat(FACTS);
    for part in [
        "letpcb=crate::process::get_process(caller);",
        "ProcessState::Zombie(_)|ProcessState::Terminated(_)",
        "letnames_now=super::reply_inbox::for_pid(caller);",
        "matchowner_of(inbox){",
        "Some(KERNEL_OWNER)=>Owner::Kernel,",
        "recorded:inbox,",
        "named:pcb.as_ref().and_then(|p|p.reply_inbox()).is_some(),",
        "reachable:crate::services::registry::caller_may_reach_pid(caller),",
        "letErr(refused)=admit(&facts)else{returntrue};",
        "REFUSED.reply_refused(&name,why(refused));",
    ] {
        assert!(f.contains(part), "the facts lost `{part}`");
    }
    assert!(f.contains("Site::new(b\"ipc.call\")"), "the refusal left the pid-free ipc.call site");
}
