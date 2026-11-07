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


//! Capability monotonicity (`Formal.capability_monotonicity`,
//! `AdminGrant.old_rule_grant_exceeds_admitted`) on the rules `sys_cap_grant`
//! and `sys_cap_revoke` apply: the caller's word and the target's envelope.

use super::caps_bits::install_caps;
use super::grant_rule::{grant_decision, may_grant, may_revoke, revoke_decision, within_envelope};
use super::words::random_words;
use crate::capabilities::Capability;

fn admin() -> u64 {
    Capability::Admin.bit()
}

/// `Effect.capGrant`'s precondition: Admin held, mask within the held word.
#[test]
fn grant_is_exactly_admin_and_a_held_mask() {
    let w = random_words(0x6a47, 3 * 50_000);
    for c in w.chunks(3) {
        let held = if c[2] & 1 == 0 { c[0] | admin() } else { c[0] };
        let mask = if c[2] & 2 == 0 { c[1] & held } else { c[1] };
        let model = held & admin() != 0 && mask & !held == 0;
        assert_eq!(may_grant(held, admin(), mask), model);
    }
}

/// `Effect.capRevoke`'s precondition: Admin held.
#[test]
fn revoke_is_exactly_admin() {
    for held in random_words(0x7e40, 50_000) {
        assert_eq!(may_revoke(held, admin()), held & admin() != 0);
    }
}

/// The old caller-only rule's ceiling step: a granter under the ceiling
/// hands over nothing above it.
#[test]
fn a_grant_never_lifts_a_target_above_the_ceiling() {
    let w = random_words(0xce11, 4 * 50_000);
    for c in w.chunks(4) {
        let ceiling = c[0] | admin();
        let caller = (c[1] | admin()) & ceiling;
        let target = c[2] & ceiling;
        let mask = c[3] & caller;
        if may_grant(caller, admin(), mask) {
            assert_eq!((target | mask) & !ceiling, 0);
        }
    }
}

/// Without Admin no grant passes, so nothing is ever added to a word.
#[test]
fn without_admin_no_grant_passes() {
    let w = random_words(0x0ad0, 2 * 50_000);
    for c in w.chunks(2) {
        assert!(!may_grant(c[0] & !admin(), admin(), c[1]));
    }
}

/// The trace of `AdminGrant.old_rule_grant_exceeds_admitted`: under the
/// caller-only rule, a capsule admitted with Admin and IPC could grant Admin
/// to one whose manifest named IPC alone. The caller-side rule still passes;
/// the whole decision now refuses it on the target's envelope.
#[test]
fn the_fixed_grant_refuses_a_bit_outside_the_targets_manifest() {
    let ipc = Capability::IPC.bit();
    let granter = admin() | ipc;
    let target_envelope = ipc;
    assert!(may_grant(granter, admin(), admin()));
    assert!(!grant_decision(Some(granter), Some(target_envelope), admin(), admin()));
}

#[test]
fn a_grant_inside_the_targets_manifest_is_allowed() {
    let ipc = Capability::IPC.bit();
    let net = Capability::Network.bit();
    let granter = admin() | ipc | net;
    let target_envelope = ipc | net;
    assert!(grant_decision(Some(granter), Some(target_envelope), admin(), net));
    assert!(grant_decision(Some(granter), Some(target_envelope), admin(), ipc | net));
}

#[test]
fn a_caller_lacking_the_bit_or_admin_is_refused() {
    let ipc = Capability::IPC.bit();
    let net = Capability::Network.bit();
    let everything = !0u64;
    assert!(!grant_decision(Some(admin() | ipc), Some(everything), admin(), net));
    assert!(!grant_decision(Some(ipc | net), Some(everything), admin(), net));
}

#[test]
fn a_grant_to_or_from_a_missing_process_is_refused() {
    let all = !0u64;
    assert!(!grant_decision(Some(all), None, admin(), Capability::IPC.bit()));
    assert!(!grant_decision(None, Some(all), admin(), Capability::IPC.bit()));
}

/// A process no manifest admitted has a zero envelope: only the empty grant
/// passes.
#[test]
fn nothing_can_be_granted_to_a_process_without_an_envelope() {
    for mask in random_words(0x0e1f, 10_000) {
        assert_eq!(grant_decision(Some(!0), Some(0), admin(), mask), mask == 0);
    }
}

/// `grant_decision` is the model's `Effect.capGrant` precondition: caller
/// holds Admin and the mask, and the mask is inside the target's envelope.
#[test]
fn the_decision_is_the_model_precondition() {
    let w = random_words(0xdec1, 4 * 50_000);
    for c in w.chunks(4) {
        let held = if c[3] & 1 == 0 { c[0] | admin() } else { c[0] };
        let mask = if c[3] & 2 == 0 { c[1] & held } else { c[1] };
        let env = if c[3] & 4 == 0 { c[2] | mask } else { c[2] };
        let model = held & admin() != 0 && mask & !held == 0 && mask & !env == 0;
        assert_eq!(grant_decision(Some(held), Some(env), admin(), mask), model);
        assert_eq!(within_envelope(mask, env), mask & !env == 0);
    }
}

/// The installed word is inside the envelope recorded beside it: what an
/// admitted capsule holds at spawn is already within its grant bound.
#[test]
fn the_installed_word_is_inside_the_envelope() {
    let w = random_words(0x1e5e, 4 * 50_000);
    for c in w.chunks(4) {
        let (req, opt, ceiling) = (c[0], c[1], c[3] | c[0] | c[1]);
        let granted = c[2] & (req | opt);
        let install = install_caps(req, opt, granted);
        let envelope = (req | opt) & ceiling;
        assert_eq!(install & !envelope, 0);
    }
}

/// `revoke_decision` is the model's `revokeDecision` on all 32 inputs.
#[test]
fn the_revoke_decision_is_the_model() {
    for b in 0u8..32 {
        let (same, admin, init, parent, protected) =
            (b & 1 != 0, b & 2 != 0, b & 4 != 0, b & 8 != 0, b & 16 != 0);
        let model = same || (admin && if protected { init } else { init || parent });
        assert_eq!(revoke_decision(same, admin, init, parent, protected), model, "{b:05b}");
    }
}

#[test]
fn a_process_may_drop_its_own_bits_without_admin() {
    assert!(revoke_decision(true, false, false, false, false));
    assert!(revoke_decision(true, false, false, false, true));
}

#[test]
fn an_admin_holder_that_is_neither_init_nor_parent_is_refused() {
    assert!(!revoke_decision(false, true, false, false, false));
    assert!(!revoke_decision(false, false, true, true, false));
}

#[test]
fn a_parent_revokes_its_child_but_not_a_protected_service() {
    assert!(revoke_decision(false, true, false, true, false));
    assert!(!revoke_decision(false, true, false, true, true));
    assert!(revoke_decision(false, true, true, false, true));
}
