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

//! Why a FIFO transaction was refused, one name per step. The letters are
//! the TCG timeouts in `regs`.

use crate::security::tpm::error::TpmError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FifoFail {
    /// Shorter than a command header; nothing was sent.
    CommandTooShort,
    /// TPM_ACCESS never showed activeLocality with tpmRegValidSts (A).
    LocalityNotGranted,
    /// commandReady never came up after asking for it twice (B each).
    NotReady,
    /// burstCount stayed zero (A).
    NoBurst,
    /// stsValid did not come back after a FIFO access (C).
    StatusNotValid,
    /// Expect dropped before the last command byte: the part saw a shorter
    /// command than was sent.
    ExpectDropped,
    /// Expect still set after the last byte: the part wants more.
    ExpectStillSet,
    /// No dataAvail after tpmGo (D).
    NoResponse,
    /// dataAvail dropped before the size the header named (C).
    ResponseStalled,
    /// Header size under ten bytes or over the caller's buffer.
    ResponseSize,
    /// dataAvail still set after the size the header named.
    ResponseTrailing,
}

impl FifoFail {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::CommandTooShort => "command shorter than its header",
            Self::LocalityNotGranted => "locality 0 not granted within 750 ms",
            Self::NotReady => "commandReady not set after two requests of 2 s",
            Self::NoBurst => "burstCount stayed zero for 750 ms",
            Self::StatusNotValid => "stsValid not set within 200 ms",
            Self::ExpectDropped => "part stopped expecting before the last byte",
            Self::ExpectStillSet => "part still expecting after the last byte",
            Self::NoResponse => "no response within 30 s of tpmGo",
            Self::ResponseStalled => "response stopped before its stated size",
            Self::ResponseSize => "response size out of range",
            Self::ResponseTrailing => "response longer than its stated size",
        }
    }

    /// The transport error the callers already understand.
    pub(crate) const fn error(self) -> TpmError {
        match self {
            Self::LocalityNotGranted | Self::NotReady | Self::NoBurst => TpmError::Timeout,
            Self::StatusNotValid | Self::NoResponse | Self::ResponseStalled => TpmError::Timeout,
            _ => TpmError::InvalidResponse,
        }
    }
}
