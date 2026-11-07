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

use crate::capabilities::{Capability, CapabilityToken};

impl CapabilityToken {
    #[inline]
    pub fn can_debug(&self) -> bool {
        self.grants(Capability::Debug) && self.is_valid()
    }
    #[inline]
    pub fn can_admin(&self) -> bool {
        self.grants(Capability::Admin) && self.is_valid()
    }
    /// The ACPI owner's calls: the firmware's tables, regions, ports and
    /// configuration space, its events, and the late PCI probe. Admin does
    /// not stand in for it.
    #[inline]
    pub fn can_acpi_owner(&self) -> bool {
        self.grants(Capability::AcpiOwner) && self.is_valid()
    }
    /// Push a policy decision to the kernel. Admin does not stand in for it.
    #[inline]
    pub fn can_policy_push(&self) -> bool {
        self.grants(Capability::PolicyPush) && self.is_valid()
    }
    /// Power the machine off or reset it. Admin does not stand in for it.
    #[inline]
    pub fn can_power_control(&self) -> bool {
        self.grants(Capability::PowerControl) && self.is_valid()
    }
    #[inline]
    pub fn can_local_sign(&self) -> bool {
        self.grants(Capability::LocalSign) && self.is_valid()
    }
    /// The right to ask for a package to be installed. Deliberately
    /// separate from `ForeignExec`: the asker never hosts anything.
    #[inline]
    pub fn can_app_install(&self) -> bool {
        self.grants(Capability::AppInstall) && self.is_valid()
    }
    #[inline]
    pub fn can_attest_read(&self) -> bool {
        self.grants(Capability::AttestRead) && self.is_valid()
    }
    /// A TPM quote names the machine permanently, so it is never handed to a
    /// capsule that holds `Network`, whatever else it holds.
    #[inline]
    pub fn can_attest_doc(&self) -> bool {
        self.grants(Capability::AttestRead) && self.is_valid() && !self.reaches_network()
    }
    /// Not a grant: the one thing that disqualifies a caller from a quote.
    #[inline]
    fn reaches_network(&self) -> bool {
        self.grants(Capability::Network)
    }
    #[inline]
    pub fn can_store_write(&self) -> bool {
        self.grants(Capability::StoreWrite) && self.is_valid()
    }
    #[inline]
    pub fn can_foreign_exec(&self) -> bool {
        self.grants(Capability::ForeignExec) && self.is_valid()
    }
    /// The right to stream a pinned file into the data volume, and no other.
    #[inline]
    pub fn can_stream_import(&self) -> bool {
        self.grants(Capability::StreamImport) && self.is_valid()
    }
    /// The right to the person's files' objects on the data volume, held by
    /// the user-files broker alone.
    #[inline]
    pub fn can_user_files(&self) -> bool {
        self.grants(Capability::UserFiles) && self.is_valid()
    }
    /// The right to receive this machine's device secret, held by nonos.prove alone.
    #[inline]
    pub fn can_device_secret(&self) -> bool {
        self.grants(Capability::DeviceSecret) && self.is_valid()
    }
}
