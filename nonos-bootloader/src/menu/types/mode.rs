// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::handoff::types::flags;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SecurityMode {
    Development,
    #[default]
    Standard,
    Hardened,
    SafeMode,
    NetworkIsolated,
    Recovery,
}

impl SecurityMode {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Development => "Development",
            Self::Standard => "Standard",
            Self::Hardened => "Hardened",
            Self::SafeMode => "Safe Mode",
            Self::NetworkIsolated => "Air-Gapped",
            Self::Recovery => "Recovery",
        }
    }
    pub const fn description(&self) -> &'static str {
        match self {
            Self::Development => "Unsigned kernel allowed; the kernel's STARK is still required",
            Self::Standard => "Signed kernel required, standard security enforced",
            Self::Hardened => "Full verification chain required, maximum security",
            Self::SafeMode => "No network, no audio, no optional apps: to find what fails",
            Self::NetworkIsolated => {
                "No network driver or service starts; nothing can reach a network"
            }
            Self::Recovery => "No network, setup skipped: a Terminal and Files to repair",
        }
    }
    pub const fn requires_signature(&self) -> bool {
        !matches!(self, Self::Development)
    }
    /* The profiles that need a TPM: it holds the rollback floor. */
    pub const fn requires_tpm(&self) -> bool {
        matches!(self, Self::Hardened | Self::NetworkIsolated)
    }
    /* The handoff flag that tells the kernel which profile to run. */
    pub const fn handoff_flag(&self) -> u64 {
        match self {
            Self::Development | Self::Standard => 0,
            Self::Hardened => flags::PROFILE_HARDENED,
            Self::SafeMode => flags::PROFILE_SAFE,
            Self::NetworkIsolated => flags::PROFILE_AIR_GAPPED,
            Self::Recovery => flags::PROFILE_RECOVERY,
        }
    }
}
