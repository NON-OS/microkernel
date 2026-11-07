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

//! `nsh_attest`'s refusals, named by the step of docs/nsh/nsh-v1-attest.md
//! the client reports them as.

use crate::AttestRefusal;
use nsh_attest::{CapsuleFault, Refusal};

/// The step that refused, in the client's terms.
pub fn refusal(r: Refusal) -> AttestRefusal {
    use Refusal::*;
    match r {
        Format(_) => AttestRefusal::Parse,
        Version(_) => AttestRefusal::Version,
        Challenge => AttestRefusal::Challenge,
        Quote(_) | Binding | Signer | Signature | Selection | PcrDigest => AttestRefusal::Quote,
        Pcr9 | Pcr8 => AttestRefusal::ReleasePcrs,
        Log(_) => AttestRefusal::Log,
        Replay(_) => AttestRefusal::Replay,
        BootApplication => AttestRefusal::BootApplication,
        Loader => AttestRefusal::Loader,
        Pcr14(CapsuleFault::PathOnly) => AttestRefusal::PathOnly,
        Pcr14(_) => AttestRefusal::NotVendor,
        Dma => AttestRefusal::Dma,
        Registry | RegistryIncomplete | CapsuleCount | NotVendor | Front | NetworkCapsule => {
            AttestRefusal::Registry
        }
        // Step 4 and the firmware half of step 5 are the client's, not run
        // here, so none of these comes back from the steps above.
        EnrolFormat | EnrolAuthority | EnrolSignature | EnrolFleet | EnrolHost | EnrolAk
        | EnrolHostKeys | EnrolExpired | EnrolRevoked | Firmware(_) => AttestRefusal::Parse,
    }
}
