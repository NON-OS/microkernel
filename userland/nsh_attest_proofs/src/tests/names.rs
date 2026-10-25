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

//! Every refusal is shown by a name no other refusal has.

use nsh_attest::{CapsuleFault, FormatFault, LogFault, QuoteFault, Refusal};

#[test]
fn every_refusal_has_its_own_name() {
    let all = [
        Refusal::Format(FormatFault::Short),
        Refusal::Version(3),
        Refusal::Challenge,
        Refusal::Quote(QuoteFault::Malformed),
        Refusal::Binding,
        Refusal::Signer,
        Refusal::Signature,
        Refusal::Selection,
        Refusal::PcrDigest,
        Refusal::EnrolFormat,
        Refusal::EnrolAuthority,
        Refusal::EnrolSignature,
        Refusal::EnrolFleet,
        Refusal::EnrolHost,
        Refusal::EnrolAk,
        Refusal::EnrolHostKeys,
        Refusal::EnrolExpired,
        Refusal::EnrolRevoked,
        Refusal::Pcr9,
        Refusal::Pcr8,
        Refusal::Pcr14(CapsuleFault::None),
        Refusal::Pcr14(CapsuleFault::PathOnly),
        Refusal::Pcr14(CapsuleFault::Developer),
        Refusal::Pcr14(CapsuleFault::LocalBuild),
        Refusal::Log(LogFault::Short),
        Refusal::Replay(4),
        Refusal::Loader,
        Refusal::Firmware(0),
        Refusal::Firmware(1),
        Refusal::Firmware(2),
        Refusal::Firmware(7),
        Refusal::Dma,
        Refusal::Registry,
        Refusal::RegistryIncomplete,
        Refusal::CapsuleCount,
        Refusal::Front,
    ];
    let names: std::collections::HashSet<_> = all.iter().map(|r| r.name()).collect();
    assert_eq!(names.len(), all.len());
}
