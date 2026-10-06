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

//! Why an enrollment step failed.

use crate::security::tpm::error::TpmError;
use crate::security::tpm::machine_key::KeyError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnrollError {
    /// The transport failed, a response did not parse, or the TPM refused a
    /// command. `KeyError::Refused` carries the response code, which says why:
    /// `TPM_RC_HANDLE` from the certificate read means no certificate, an
    /// integrity error from activation means the credential names another key.
    Key(KeyError),
    /// A challenge input is empty or longer than the TPM structure it fills,
    /// or the certificate index is longer than [`super::consts::CERT_MAX`].
    OutOfBounds,
    /// The TPM gave a null ticket for the message because it begins with
    /// `TPM_GENERATED_VALUE`. Signed, it could pass for an attestation the TPM
    /// built, so the restricted key will not sign it.
    Unsignable,
}

impl From<KeyError> for EnrollError {
    fn from(e: KeyError) -> Self {
        EnrollError::Key(e)
    }
}

impl From<TpmError> for EnrollError {
    fn from(e: TpmError) -> Self {
        EnrollError::Key(KeyError::Tpm(e))
    }
}

impl EnrollError {
    pub const fn as_str(self) -> &'static str {
        match self {
            EnrollError::Key(e) => e.as_str(),
            EnrollError::OutOfBounds => "enrollment input or certificate out of bounds",
            EnrollError::Unsignable => "message shaped like a tpm attestation",
        }
    }
}
