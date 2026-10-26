// NONOS Operating System (AGPL-3.0-or-later)

//! Each step of a join has its own time: a step left unanswered ends the join
//! there, named; each step starts its own clock; SAE has its longer time and
//! ends with why its last frame from the access point was discarded.

use super::frames::{assoc_resp, open_auth_ok, sae_from_ap};
use super::sim::{beacon, join, rsne, AP, STA};
use crate::ap_sim::AP_RSNE_MIXED;
use nonos_wifi_core::mlme::{MlmeFailure, MlmeState, ASSOC_MS, AUTH_MS, FOURWAY_MS, SAE_MS};
use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::sae::SaeFailure;

const T0: u64 = 10_000;

#[test]
fn a_step_left_unanswered_ends_at_its_own_deadline_named() {
    let mut m = join(JoinPolicy::ANY);
    assert_eq!(m.deadline(T0), None, "watching beacons has none");
    m.on_mgmt(&beacon(AP, &[&rsne(&[2], 0, 4)]));
    assert_eq!(m.deadline(T0), Some(T0 + AUTH_MS));
    assert!(!m.expire(T0 + AUTH_MS - 1));
    assert_eq!(m.state(), MlmeState::Authenticating);
    assert!(m.expire(T0 + AUTH_MS));
    let timeout = Some(MlmeFailure::Timeout(MlmeState::Authenticating));
    assert_eq!((m.state(), m.failure(), m.expired()), (MlmeState::Failed, timeout, true));
    assert_eq!(m.deadline(T0 + 2 * AUTH_MS), None, "a failed join has none");
}

#[test]
fn each_step_starts_its_own_clock() {
    let mut m = join(JoinPolicy::ANY);
    m.on_mgmt(&beacon(AP, &[&rsne(&[2], 0, 4)]));
    m.expire(T0);
    m.on_mgmt(&open_auth_ok(AP, STA));
    let t1 = T0 + AUTH_MS - 1;
    assert_eq!(m.deadline(t1), Some(t1 + ASSOC_MS), "from when association began");
    m.on_mgmt(&assoc_resp(0));
    let t2 = t1 + ASSOC_MS - 1;
    assert_eq!(m.deadline(t2), Some(t2 + FOURWAY_MS), "from when the handshake began");
    assert!(!m.expire(t2 + FOURWAY_MS - 1) && !m.expired());
    assert!(m.expire(t2 + FOURWAY_MS));
    assert_eq!(m.failure(), Some(MlmeFailure::Timeout(MlmeState::FourWay)));
}

#[test]
fn sae_has_its_longer_time_and_ends_with_why_its_last_frame_was_discarded() {
    let mut m = join(JoinPolicy::ANY);
    assert!(m.on_mgmt(&beacon(AP, &[&AP_RSNE_MIXED])).tx.is_some(), "the commit");
    assert_eq!(m.deadline(T0), Some(T0 + SAE_MS));
    assert!(!m.expire(T0 + AUTH_MS), "past Open System's time, inside SAE's");
    m.on_mgmt(&sae_from_ap(1, 77, &[]));
    assert!(m.expire(T0 + SAE_MS));
    assert_eq!(m.failure(), Some(MlmeFailure::Sae(SaeFailure::GroupNotSupported)));
    assert!(m.expired() && m.state() == MlmeState::Failed);
}

#[test]
fn a_refused_join_is_not_one_that_ran_out_of_time() {
    let mut m = join(JoinPolicy::ANY);
    m.on_mgmt(&beacon(AP, &[&rsne(&[2], 0, 4)]));
    m.on_mgmt(&open_auth_ok(AP, STA));
    m.on_mgmt(&assoc_resp(17));
    assert_eq!((m.state(), m.expired()), (MlmeState::Failed, false));
    assert!(!m.expire(u64::MAX), "nothing left to expire");
}
