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

//! The supervisor's caps are the plan's two values, picked by the policy's
//! NET_ANON bit. A run role still holding LocalSign was built before C5a.

use nonos_proof_abi::trust::{supervisor_caps, LINUX_CAPS, LINUX_NET_CAPS, NET_ANON};
use nonos_proof_abi::{tag, verify, Refusal};

use crate::fixture::{expected, honest};

#[test]
fn the_two_values_are_plan_11s() {
    assert_eq!((LINUX_CAPS, LINUX_NET_CAPS), (0x1_0400_1979, 0x1_0400_197D));
    assert_eq!(supervisor_caps(0x21), LINUX_CAPS);
    assert_eq!(supervisor_caps(0x21 | NET_ANON), LINUX_NET_CAPS);
}

#[test]
fn caps_from_before_c5a_or_for_another_role_are_refused() {
    let (mut f, mut parts, pinned) = honest();
    let pins = pinned.trust();
    let local_sign = 1u64 << 33;
    for caps in [LINUX_CAPS | local_sign, LINUX_NET_CAPS | local_sign, LINUX_NET_CAPS, 0] {
        f.record.sup_caps = caps;
        parts.set(tag::RECORD, f.record.encode().to_vec());
        let got = verify(&parts.bytes(), &expected(&f.policy, &pins));
        assert_eq!(got, Err(Refusal::Supervisor), "{caps:#x}");
    }
}

#[test]
fn a_net_anon_policy_wants_the_run_net_caps() {
    let (mut f, mut parts, pinned) = honest();
    let pins = pinned.trust();
    let mut net = f.policy;
    net[48] |= NET_ANON as u8;
    let got = verify(&parts.bytes(), &expected(&net, &pins));
    assert_eq!(got, Err(Refusal::Supervisor), "run-role caps under NET_ANON");
    f.record.sup_caps = LINUX_NET_CAPS;
    parts.set(tag::RECORD, f.record.encode().to_vec());
    // Past the supervisor step, the changed record no longer binds the proof.
    let got = verify(&parts.bytes(), &expected(&net, &pins));
    assert_eq!(got, Err(Refusal::DeviceBinding));
}
