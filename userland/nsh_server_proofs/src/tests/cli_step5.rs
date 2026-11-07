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

//! Step 5's refusals are each told apart: the log that does not parse, the
//! PCR it does not replay to, the boot application or loader PCR 4 names,
//! and the capsules PCR 14 records, path-only apart from the rest, so a
//! client refused by a development image says that, and only that.

use nsh_attest::{CapsuleFault, LogFault, Refusal};
use nsh_client::{attest_refusal, AttestRefusal as A};

#[test]
fn each_step5_refusal_has_its_own_answer() {
    assert_eq!(attest_refusal(Refusal::Log(LogFault::Short)), A::Log);
    assert_eq!(attest_refusal(Refusal::Log(LogFault::Capsule)), A::Log);
    for pcr in [0u8, 4, 7, 14] {
        assert_eq!(attest_refusal(Refusal::Replay(pcr)), A::Replay);
    }
    assert_eq!(attest_refusal(Refusal::BootApplication), A::BootApplication);
    assert_eq!(attest_refusal(Refusal::Loader), A::Loader);
    assert_eq!(attest_refusal(Refusal::Pcr14(CapsuleFault::PathOnly)), A::PathOnly);
    for fault in [CapsuleFault::None, CapsuleFault::Developer, CapsuleFault::LocalBuild] {
        assert_eq!(attest_refusal(Refusal::Pcr14(fault)), A::NotVendor, "{fault:?}");
    }
}

#[test]
fn the_other_steps_answer_as_before() {
    assert_eq!(attest_refusal(Refusal::Challenge), A::Challenge);
    assert_eq!(attest_refusal(Refusal::Pcr8), A::ReleasePcrs);
    assert_eq!(attest_refusal(Refusal::Pcr9), A::ReleasePcrs);
    assert_eq!(attest_refusal(Refusal::Dma), A::Dma);
    assert_eq!(attest_refusal(Refusal::Front), A::Registry);
    assert_eq!(attest_refusal(Refusal::NotVendor), A::Registry);
}
