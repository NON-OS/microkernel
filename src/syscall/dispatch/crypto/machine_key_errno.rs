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

//! Which errno a caller of `CryptoMachineKey` gets for each way the TPM
//! can fail.

use crate::security::tpm::error::TpmError;
use crate::security::tpm::machine_key::KeyError;

/// `TPM_RC_POLICY_FAIL` as a format-1 code with the handle, parameter or
/// session number stripped: bits 8 to 11 carry that number, bit 6 says
/// which kind, and neither is part of the error.
const RC_POLICY_FAIL: u32 = 0x09D;
const RC_FORMAT1_ERROR_MASK: u32 = 0x0BF;

/// No TPM, a TPM refusing because the machine is not in the key's state, or a
/// transport fault: a caller unlocking a volume says a different thing for each.
pub(super) fn errno_for(e: KeyError) -> i32 {
    match e {
        KeyError::Tpm(TpmError::NotPresent) => 19,
        KeyError::Tpm(TpmError::Timeout) => 110,
        KeyError::Tpm(TpmError::InvalidResponse) => 5,
        KeyError::Refused(rc) if rc & RC_FORMAT1_ERROR_MASK == RC_POLICY_FAIL => 13,
        KeyError::Refused(_) => 5,
        KeyError::BadLabel => 22,
        KeyError::NoRandomness => 5,
    }
}
