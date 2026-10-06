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

//! What each enrollment failure tells the caller.

use crate::syscall::microkernel::errnos::{
    ERRNO_ACCES, ERRNO_INVAL, ERRNO_IO, ERRNO_NODEV, ERRNO_NOENT, ERRNO_TIMEDOUT,
};
use crate::security::tpm::enroll::EnrollError;
use crate::security::tpm::error::TpmError;
use crate::security::tpm::machine_key::KeyError;

/// `TPM_RC_HANDLE` on the first handle: no certificate at this EK's index.
const RC_NO_SUCH_HANDLE: u32 = 0x18B;

/// ENODEV with no TPM, ETIMEDOUT or EIO when it answered late or badly,
/// ENOENT when it holds no such object, EACCES when it refused the command
/// (a credential for another key among them), EINVAL for an input or a
/// certificate out of bounds and a message the AK will not sign.
pub(super) fn errno(e: EnrollError) -> i64 {
    match e {
        EnrollError::Key(KeyError::Tpm(TpmError::NotPresent)) => ERRNO_NODEV,
        EnrollError::Key(KeyError::Tpm(TpmError::Timeout)) => ERRNO_TIMEDOUT,
        EnrollError::Key(KeyError::Tpm(TpmError::InvalidResponse)) => ERRNO_IO,
        EnrollError::Key(KeyError::Refused(RC_NO_SUCH_HANDLE)) => ERRNO_NOENT,
        EnrollError::Key(KeyError::Refused(_)) => ERRNO_ACCES,
        EnrollError::Key(KeyError::BadLabel) => ERRNO_INVAL,
        EnrollError::OutOfBounds | EnrollError::Unsignable => ERRNO_INVAL,
    }
}
