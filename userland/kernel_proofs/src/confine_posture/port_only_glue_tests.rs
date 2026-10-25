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

//! Where the port-only rule (H-02) is used: the group check asks it for the
//! claim and each holder, the group marks members across a shared bus, the
//! capsule records it, and the claim closes Memory Space for one; with no
//! unit to confine it, every verdict is asked with it (H-02b, port_only_verdict_glue_tests.rs).

const JOIN_GROUP: &str = include_str!("../../../../src/hardware/broker/confine/join_group.rs");
const GATES: &str = include_str!("../../../../src/hardware/broker/confine/gates.rs");
const ACS: &str = include_str!("../../../../src/hardware/broker/confine/acs.rs");
const ONE: &str = include_str!("../../../../src/hardware/broker/confine/one.rs");
const ATTACH: &str = include_str!("../../../../src/hardware/broker/confine/attach.rs");
const CLAIM: &str = include_str!("../../../../src/hardware/broker/claim/claim.rs");

#[test]
fn the_group_check_asks_the_rule_both_ways_for_the_claim_and_each_holder() {
    assert!(JOIN_GROUP.contains("let claim_port_only = all[pos].port_only;"));
    assert!(JOIN_GROUP.contains("let ports = (claim_port_only, c.port_only);"));
    assert!(JOIN_GROUP.contains("blocked_by(claimed, group, &held, ports, place)"));
    assert!(JOIN_GROUP.contains("filter_map(|&e| Some((e, peer(e)?)))"), "no held device's group");
    assert!(JOIN_GROUP.contains("(p.at, p.group.as_deref())"), "an unread group taken as empty");
    assert!(JOIN_GROUP.contains("Blocked::Unreadable(at) => unreadable_line(at, &holder(c.pid))"));
    assert!(GATES.contains("None => Peer { at, group: None, aliases: Vec::new() },"));
}

#[test]
fn a_held_device_carrying_the_claimed_requester_id_refuses_it() {
    let check = "if let Some((_, p)) = peers.iter().find(|(_, p)| p.aliases.contains(&address)) {";
    assert!(JOIN_GROUP.contains(check), "the reverse alias check is not as written");
    assert!(JOIN_GROUP.contains("peer_alias_line(p.at, &holder(c.pid))"));
}

#[test]
fn members_across_a_shared_bus_are_marked_and_the_capsule_records_its_claim() {
    assert!(ACS.contains("let across = shared(&hops);"));
    assert!(ACS.contains("shared: across.contains(&at)"));
    assert!(ONE.contains("capsule_for(&mut all, pid, device_id, (must, port_only))"));
    assert!(ATTACH.contains("attach_one(pid, device_id, address, (false, port_only)"));
    assert!(ONE.contains("port_only(has(Capability::Dma), has(Capability::Mmio))"));
}

#[test]
fn a_port_only_claim_closes_memory_space_before_the_device_is_powered() {
    assert!(ATTACH.contains("attach_as(pid, device_id, port_only).map(|()| port_only)"));
    let close = CLAIM.find("super::port_only::port_only_quietly(device_id);").expect("not closed");
    let gate = CLAIM.find("if port_only {").expect("closed for every claim");
    let power = CLAIM.find("power_on_device(device_id)").expect("never powered");
    assert!(gate < close && close < power, "{gate} {close} {power}");
}
