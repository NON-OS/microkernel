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

//! musl's resolver reads its answers with recvmsg. A resolver descriptor
//! holds no socket, so recvmsg asks it first and answers from the resolver,
//! with the nameserver's address in the message's name, before any socket
//! lookup could refuse it. The booted lookup is the proof that it answers.

const RECV: &str = include_str!("../../../../capsule_linux/src/linux/net/msg/recv.rs");
const RESOLVER: &str = include_str!("../../../../capsule_linux/src/linux/net/msg/recv_resolver.rs");

#[test]
fn recvmsg_answers_a_resolver_before_it_looks_for_a_socket() {
    let asked = RECV.find("if super::super::dgram::is_resolver(guest, fd) {").expect("asked");
    assert!(asked < RECV.find("sock_of(guest, fd)").unwrap());
    assert!(RECV.contains("return super::recv_resolver::recvmsg(guest, fd, msg);"));
}

#[test]
fn the_answer_carries_its_sender_and_no_control_data() {
    assert!(RESOLVER.contains("super::super::resolver::answer(guest, fd, base, len, name, lenp)"));
    assert!(RESOLVER.contains("(h.name, msg + NAMELEN_AT)"));
    assert!(RESOLVER.contains("guest.write(msg + CONTROLLEN_AT, &0u64.to_le_bytes())"));
    assert!(RESOLVER.contains("if want.is_some_and(|w| w > len) { MSG_TRUNC as u32 } else { 0 }"));
}
